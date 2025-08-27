//! This module contains the implementation for the RTT protocol. It's not meant to be used directly
//! in user code, and therefore mostly undocumented. The module is only public so that it can be
//! accessed from the rtt_init! macro.

use crate::ChannelMode;
use core::cmp::min;
use core::fmt;
use core::ptr;
use portable_atomic::{AtomicUsize, Ordering::SeqCst};

// Note: this is zero-initialized in the initialization macro so all zeros must be a valid value
#[repr(C)]
pub struct RttHeader {
    id: [u8; 16],
    max_up_channels: usize,
    max_down_channels: usize,
    // Followed in memory by:
    // up_channels: [Channel; max_up_channels]
    // down_channels: [Channel; down_up_channels]
}

impl RttHeader {
    /// Initializes the control block header.
    ///
    /// # Safety
    ///
    /// The arguments must correspond to the sizes of the arrays that follow the header in memory.
    pub unsafe fn init(&mut self, max_up_channels: usize, max_down_channels: usize) {
        ptr::write_volatile(&mut self.max_up_channels, max_up_channels);
        ptr::write_volatile(&mut self.max_down_channels, max_down_channels);

        // Copy the ID backward to avoid storing the magic string in the binary. The ID is
        // written backwards to make it less likely an unfinished control block is detected by the host.

        const MAGIC_STR_BACKWARDS: &[u8; 16] = b"\0\0\0\0\0\0TTR REGGES";

        for (idx, byte) in MAGIC_STR_BACKWARDS.into_iter().enumerate() {
            ptr::write_volatile(&mut self.id[15 - idx], *byte);
        }
    }

    pub fn max_up_channels(&self) -> usize {
        self.max_up_channels
    }
}

// Note: this is zero-initialized in the initialization macro so all zeros must be a valid value
#[repr(C)]
pub struct RttChannel {
    name: *const u8,
    buffer: *mut u8,
    size: usize,
    write: AtomicUsize,
    read: AtomicUsize,
    flags: AtomicUsize,
}

impl RttChannel {
    /// Initializes the channel.
    ///
    /// # Safety
    ///
    /// The pointer arguments must point to a valid null-terminated name and writable buffer.
    pub unsafe fn init(&mut self, name: *const u8, mode: ChannelMode, buffer: *mut [u8]) {
        ptr::write_volatile(&mut self.name, name);
        ptr::write_volatile(&mut self.size, (*buffer).len());
        self.set_mode(mode);

        // Set buffer last as it can be used to detect if the channel has been initialized
        ptr::write_volatile(&mut self.buffer, buffer as *mut u8);
    }

    /// Returns true on a non-null value of the (raw) buffer pointer
    pub fn is_initialized(&self) -> bool {
        !self.buffer.is_null()
    }

    pub(crate) fn mode(&self) -> ChannelMode {
        let mode = self.flags.load(SeqCst) & 3;

        match mode {
            0 => ChannelMode::NoBlockSkip,
            1 => ChannelMode::NoBlockTrim,
            2 => ChannelMode::BlockIfFull,
            _ => ChannelMode::NoBlockSkip,
        }
    }

    pub(crate) fn set_mode(&self, mode: ChannelMode) {
        self.flags
            .store((self.flags.load(SeqCst) & !3) | mode as usize, SeqCst);
    }

    // This method should only be called for down channels.
    pub(crate) fn read(&self, mut buf: &mut [u8]) -> usize {
        let (write, mut read) = self.read_pointers();

        let mut total = 0;

        // Read while buffer contains data and output buffer has space (maximum of two iterations)
        while !buf.is_empty() {
            let count = min(self.readable_contiguous(write, read), buf.len());
            if count == 0 {
                break;
            }

            unsafe {
                ptr::copy_nonoverlapping(self.buffer.add(read), buf.as_mut_ptr(), count);
            }

            total += count;
            read += count;

            if read >= self.size {
                // Wrap around to start
                read = 0;
            }

            buf = &mut buf[count..];
        }

        self.read.store(read, SeqCst);

        total
    }

    /// This method should only be called for up channels.
    pub(crate) fn writer(&self) -> RttWriter<'_> {
        RttWriter {
            chan: self,
            write: self.read_pointers().0,
            total: 0,
            state: WriteState::Writable,
        }
    }

    /// Gets the amount of contiguous data available for reading
    fn readable_contiguous(&self, write: usize, read: usize) -> usize {
        if read > write {
            self.size - read
        } else {
            write - read
        }
    }

    pub(crate) fn read_pointers(&self) -> (usize, usize) {
        let write = self.write.load(SeqCst);
        let read = self.read.load(SeqCst);

        if write >= self.size || read >= self.size {
            // Pointers have been corrupted. This doesn't happen in well-behaved programs, so
            // attempt to reset the buffer.

            self.write.store(0, SeqCst);
            self.read.store(0, SeqCst);
            return (0, 0);
        }

        (write, read)
    }
}

/// A cancellable write operation to an RTT channel.
pub(crate) struct RttWriter<'c> {
    chan: &'c RttChannel,
    write: usize,
    total: usize,
    state: WriteState,
}

#[derive(Eq, PartialEq)]
enum WriteState {
    /// Operation can continue
    Writable,

    /// Buffer space ran out but the written data will still be committed
    Full,

    /// The operation failed and won't be committed, or it has already been committed.
    Finished,
}

impl RttWriter<'_> {
    pub fn write(&mut self, buf: &[u8]) {
        self.write_with_mode(self.chan.mode(), buf);
    }

    pub fn write_with_mode(&mut self, mode: ChannelMode, mut buf: &[u8]) {
        while self.state == WriteState::Writable && !buf.is_empty() {
            let count = min(self.writable_contiguous(), buf.len());

            if count == 0 {
                // Buffer is full

                match mode {
                    ChannelMode::NoBlockSkip => {
                        // Mark the entire operation as failed if even one part cannot be written in
                        // full.
                        self.state = WriteState::Finished;
                        return;
                    }

                    ChannelMode::NoBlockTrim => {
                        // If the buffer is full, write as much as possible (note: no return), and
                        // mark the operation as full, which prevents further writes.
                        self.state = WriteState::Full;
                    }

                    ChannelMode::BlockIfFull => {
                        // Commit everything written so far and spin until more can be written
                        self.chan.write.store(self.write, SeqCst);
                        continue;
                    }
                }
            }

            unsafe {
                ptr::copy_nonoverlapping(buf.as_ptr(), self.chan.buffer.add(self.write), count);
            }

            self.write += count;
            self.total += count;

            if self.write >= self.chan.size {
                // Wrap around to start
                self.write = 0;
            }

            buf = &buf[count..];
        }
    }

    /// Gets the amount of contiguous space available for writing
    fn writable_contiguous(&self) -> usize {
        let read = self.chan.read_pointers().1;

        if read > self.write {
            read - self.write - 1
        } else if read == 0 {
            self.chan.size - self.write - 1
        } else {
            self.chan.size - self.write
        }
    }

    pub fn is_failed(&self) -> bool {
        self.state != WriteState::Finished
    }

    pub fn commit(mut self) -> usize {
        self.commit_impl();

        self.total
    }

    fn commit_impl(&mut self) {
        match self.state {
            WriteState::Finished => (),
            WriteState::Full | WriteState::Writable => {
                // Commit the write pointer so the host can see the new data
                self.chan.write.store(self.write, SeqCst);
                self.state = WriteState::Finished;
            }
        }
    }
}

impl Drop for RttWriter<'_> {
    fn drop(&mut self) {
        self.commit_impl();
    }
}

impl fmt::Write for RttWriter<'_> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.write(s.as_bytes());
        Ok(())
    }
}
