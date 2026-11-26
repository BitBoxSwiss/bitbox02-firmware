// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::{Eeprom, Hal, Memory, System};
use bitbox02_rust::hww::transport::{HwwTransport, hww_transport};
use std::io::{self, Read, Write};
use std::marker::PhantomData;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::os::fd::AsRawFd;
use std::rc::Rc;
use std::time::{Duration, Instant};
use util::bb02_async::{Task, block_on};

pub const TICK_INTERVAL: Duration = Duration::from_millis(5);
pub const MNEMONIC: &str = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
const REPORTS_PER_TICK: usize = 64;

struct Connection {
    stream: TcpStream,
    input: [u8; 64],
    received: usize,
    output: Option<[u8; 64]>,
    sent: usize,
}

impl Connection {
    fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nonblocking(true)?;
        stream.set_nodelay(true)?;
        Ok(Self {
            stream,
            input: [0; 64],
            received: 0,
            output: None,
            sent: 0,
        })
    }

    fn read<H: Hal + Default + 'static>(
        &mut self,
        transport: &mut HwwTransport<H>,
        now_ms: u64,
    ) -> io::Result<()> {
        for _ in 0..REPORTS_PER_TICK {
            match self.stream.read(&mut self.input[self.received..]) {
                Ok(0) => return Err(io::ErrorKind::UnexpectedEof.into()),
                Ok(len) => {
                    self.received += len;
                    if self.received == self.input.len() {
                        transport.handle_report(&self.input, now_ms);
                        self.received = 0;
                        if crate::hal::reset_requested() {
                            break;
                        }
                    }
                }
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
                Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    fn write<H: Hal + Default + 'static>(
        &mut self,
        transport: &mut HwwTransport<H>,
    ) -> io::Result<()> {
        for _ in 0..REPORTS_PER_TICK {
            if self.output.is_none() {
                self.output = transport.pull_report();
                self.sent = 0;
            }
            let Some(report) = self.output.as_ref() else {
                break;
            };
            match self.stream.write(&report[self.sent..]) {
                Ok(0) => return Err(io::ErrorKind::WriteZero.into()),
                Ok(len) => {
                    self.sent += len;
                    if self.sent == report.len() {
                        self.output = None;
                    }
                }
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => break,
                Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    fn abortive_close(&self) -> io::Result<()> {
        // Avoid leaving the listening port unavailable after termination with an active client.
        let linger = libc::linger {
            l_onoff: 1,
            l_linger: 0,
        };
        let result = unsafe {
            libc::setsockopt(
                self.stream.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_LINGER,
                (&linger as *const libc::linger).cast(),
                std::mem::size_of_val(&linger) as libc::socklen_t,
            )
        };
        if result == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}

pub struct SimulatorCore<H: Hal + Default + 'static> {
    listener: TcpListener,
    connection: Option<Connection>,
    transport: HwwTransport<H>,
    startup_task: Option<Task<'static, ()>>,
    started_at: Instant,
    headless: bool,
    // Firmware globals must only be accessed from the owning event-loop thread.
    _single_thread: PhantomData<Rc<()>>,
}

fn log(headless: bool, message: &str) {
    if headless {
        bitbox02::print_stdout(&format!("{message}\n"));
    } else {
        tracing::info!("{message}");
    }
}

impl<H: Hal + Default + 'static> SimulatorCore<H> {
    pub fn new(args: &crate::Args) -> io::Result<Self> {
        let headless = args.nogui;
        log(headless, "USB setup success");
        log(headless, "HWW setup success");
        if !bitbox02::sd::format() {
            return Err(io::Error::other("ERROR, sd card setup failed"));
        }
        log(headless, "Sd card setup success");
        bitbox02::testing::mock_memory();
        bitbox02::memory::fake_nova();
        log(headless, "Memory setup success");

        let mut hal = H::default();
        block_on(bitbox02_rust::secp256k1::init(&mut hal))
            .map_err(|_| io::Error::other("Failed to initialize secp256k1"))?;
        if args.preseed {
            let seed = bitbox02_rust::bip39::mnemonic_to_seed(MNEMONIC).unwrap();
            block_on(bitbox02_rust::keystore::encrypt_and_store_seed(
                &mut hal, &seed, "",
            ))
            .map_err(|_| io::Error::other("Failed to preseed"))?;
            hal.memory().set_initialized().unwrap();
        }
        hal.eeprom().setup();
        hal.eeprom().init();

        let listener = TcpListener::bind(("0.0.0.0", args.port))?;
        listener.set_nonblocking(true)?;
        log(
            headless,
            &format!("Listening on port {}", listener.local_addr()?.port()),
        );
        Ok(Self {
            listener,
            connection: None,
            transport: hww_transport::<H>(),
            startup_task: Some(Box::pin(H::System::startup())),
            started_at: Instant::now(),
            headless,
            _single_thread: PhantomData,
        })
    }

    fn disconnect(&mut self) {
        if self.connection.take().is_some() {
            bitbox02_rust::hww::reset_session(&mut H::default());
            self.transport = hww_transport::<H>();
            log(self.headless, "Socket connection closed");
            log(
                self.headless,
                "Waiting for new clients, CTRL+C to shut down the simulator",
            );
        }
    }

    pub fn tick(&mut self) -> io::Result<bool> {
        if crate::signals::quitting() {
            if let Some(connection) = self.connection.as_ref() {
                log(self.headless, "\nGot term/int signal, exiting");
                connection.abortive_close()?;
            } else {
                log(self.headless, "\nGot signal, exiting");
            }
            return Ok(false);
        }
        if crate::signals::take_hangup() && self.connection.is_some() {
            log(self.headless, "\nGot hangup signal, disconnecting");
            self.disconnect();
        }
        if let Some(task) = self.startup_task.as_mut()
            && util::bb02_async::spin(task).is_ready()
        {
            self.startup_task = None;
        }
        // Like accept() in the C simulator, leave additional clients queued until the current
        // connection closes. No firmware or UI work runs on socket worker threads.
        if self.connection.is_none() && self.startup_task.is_none() {
            match self.listener.accept() {
                Ok((stream, _)) => {
                    self.connection = Some(Connection::new(stream)?);
                    log(self.headless, "Socket connection setup success");
                }
                Err(err)
                    if matches!(
                        err.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                    ) => {}
                Err(err) => return Err(err),
            }
        }
        let now_ms = self.started_at.elapsed().as_millis() as u64;
        if let Some(connection) = self.connection.as_mut()
            && connection.read(&mut self.transport, now_ms).is_err()
        {
            self.disconnect();
        }
        self.transport.tick(now_ms);
        bitbox02_rust::async_usb::spin();
        if crate::hal::take_reset_request() {
            self.disconnect();
            let mut hal = H::default();
            hal.eeprom().setup();
            hal.eeprom().init();
            if !self.headless {
                bitbox02::ui::screen_process_waiting_switch_to_lockscreen();
            }
            return Ok(true);
        }
        if let Some(connection) = self.connection.as_mut()
            && connection.write(&mut self.transport).is_err()
        {
            self.disconnect();
        }
        Ok(true)
    }
}

impl<H: Hal + Default + 'static> Drop for SimulatorCore<H> {
    fn drop(&mut self) {
        self.startup_task.take();
        bitbox02_rust::hww::reset_session(&mut H::default());
    }
}
