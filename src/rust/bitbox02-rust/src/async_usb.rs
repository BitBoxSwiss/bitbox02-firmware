// Copyright 2020 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module provides the executor for tasks that are spawned with an API request and deliver a
//! USB response. Terminology: host = computer, device = BitBox02.

use crate::bb02_async::{option, spin as spin_task, Task};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::task::Poll;

type UsbOut = Vec<u8>;
type UsbIn = Vec<u8>;

/// If a task is running (see `UsbTaskState`), this state is active and manages waiting for another
/// request from the host in an async fashion. This allows to have multi-request workflows in async.
///
/// Normal flow: request(A) -> response(A)
/// Multirequess flow: request(A) -> response(WANT_B) -> request(B) -> response(A)
enum WaitingForNextRequestState {
    /// We are not waiting for another request from the host. This is the default state when
    /// processing a request.
    Idle,
    /// Since we have a strict request<->response model, we always need to send a response before
    /// getting another request. In this state, we are ready to send a response and are waiting for
    /// the host to fetch it.
    SendingResponse(UsbOut),
    /// Host got the response, now we are waiting for the next request by the host.
    AwaitingRequest,
}

/// A safer version of `Option<UsbIn>`. RefCell so we cannot accidentally borrow illegally.
struct SafeNextRequest(RefCell<Option<UsbIn>>);

/// Safety: this implements Sync even though it is not thread safe. This is okay, as we
/// run only in a single thread in the BitBox02.
unsafe impl Sync for SafeNextRequest {}

/// An option resolving the `next_request()` future. It is `Some(...)` once a request we've been
/// waiting for arrives. See `next_requset()` for more details.
static NEXT_REQUEST: SafeNextRequest = SafeNextRequest(RefCell::new(None));

/// Describes the global state of an api query. The documentation of
/// the variants apply to the HWW stack, but have analogous meaning in
/// the U2F stack.
enum UsbTaskState<'a> {
    /// Waiting for a new query, nothing to do.
    Nothing,
    /// A query came in which launched a task, which is now running (e.g. user is entering a
    /// password). The option inside is `Some`, but is `None` for the brief period during which the
    /// task is polled. This is akin to popping off the task from the executor state and pushing it
    /// back in after it is polled, which is best practice (and often done via a message passing
    /// queue). This allows for the execeutor state to not be borrowed while the task is being
    /// executed, which allows the task itself to modify the executor state (otherwise we would have
    /// an illegal double-borrow of the state).
    ///
    /// The second element manages waiting for another request while processing a request, allowing
    /// multi-request workflows.
    Running(Option<Task<'a, UsbOut>>, WaitingForNextRequestState),
    /// The task has finished and written the result, so the USB response is available. We are now
    /// waiting for the host to fetch it (HWW_REQ_RETRY). For short-circuited or non-async api
    /// calls, the result might be returned immediately in response to HWW_REQ_NEW.
    ResultAvailable(UsbOut),
}

/// A safer version of UsbTaskState. RefCell so we cannot accidentally borrow illegally.
struct SafeUsbTaskState(RefCell<UsbTaskState<'static>>);

/// Safety: this implements Sync even though it is not thread safe. This is okay, as we
/// run only in a single thread in the BitBox02.
unsafe impl Sync for SafeUsbTaskState {}

/// Executor main state. Currently we only have at most one task at a time (usb api processing
/// task).
static USB_TASK_STATE: SafeUsbTaskState = SafeUsbTaskState(RefCell::new(UsbTaskState::Nothing));

/// Spawn a task to be spinned by the executor. This moves the state
/// from Nothing to Running.
///
/// *panics* - can only be called if the state is Nothing, otherwise panics.
pub fn spawn<F>(workflow: fn(UsbIn) -> F, usb_in: &[u8])
where
    F: core::future::Future<Output = UsbOut> + 'static,
{
    let mut state = USB_TASK_STATE.0.borrow_mut();
    match *state {
        UsbTaskState::Nothing => {
            let task: Task<UsbOut> = Box::pin(workflow(usb_in.to_vec()));

            *state = UsbTaskState::Running(Some(task), WaitingForNextRequestState::Idle);
        }
        _ => panic!("spawn: wrong state"),
    }
}

/// Returns true if a request is being processed and waiting for another request via the
/// `next_request()` future.
pub fn waiting_for_next_request() -> bool {
    matches!(
        *USB_TASK_STATE.0.borrow(),
        UsbTaskState::Running(Some(_), WaitingForNextRequestState::AwaitingRequest)
    )
}

/// Resolves the `next_request()` future. `waiting_for_next_request()` must be true when calling
/// this, otherwise this function panics.
pub fn on_next_request(usb_in: &[u8]) {
    let mut state = USB_TASK_STATE.0.borrow_mut();
    match *state {
        UsbTaskState::Running(
            Some(_),
            ref mut next_request_state @ WaitingForNextRequestState::AwaitingRequest,
        ) => {
            // Resolve NEXT_REQUEST future.
            *NEXT_REQUEST.0.borrow_mut() = Some(usb_in.to_vec());

            *next_request_state = WaitingForNextRequestState::Idle;
        }
        _ => panic!("on_next_request: wrong state"),
    }
}

/// Spin the currently running task, if there is one. Otherwise do
/// nothing. This is supposed to be called from the mainloop.
///
/// If this spin finishes the task, the state is moved to
/// `ResultAvailable`, which contains the result.
pub fn spin() {
    // Pop task before polling, so that USB_TASK_STATE does not stay borrowed during the poll.
    let mut popped_task = match *USB_TASK_STATE.0.borrow_mut() {
        // Illegal state, `None` is only valid during the poll.
        UsbTaskState::Running(None, _) => panic!("task not found"),
        // Get the task out, putting `None` in. This allows us to release the mutable borrow on the
        // state.
        UsbTaskState::Running(ref mut task @ Some(_), _) => task.take(),
        // Nothing to do.
        _ => None,
    };
    if let Some(ref mut task) = popped_task {
        match spin_task(task) {
            Poll::Ready(result) => {
                *USB_TASK_STATE.0.borrow_mut() = UsbTaskState::ResultAvailable(result);
            }
            Poll::Pending => {
                // Not done yet, put the task back for execution.
                if let UsbTaskState::Running(ref mut task @ None, _) =
                    *USB_TASK_STATE.0.borrow_mut()
                {
                    *task = popped_task;
                } else {
                    panic!("spin: illegal executor state");
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CopyResponseErr {
    NotRunning,
    NotReady,
}

/// To be called in response to the host asking for the result of a
/// task.
///
/// If a result is available (state = ResultAvailable), this copies
/// the usb response to `dst` and moves the state to `Nothing`, and
/// returns the Ok(<number of bytes written>).
///
/// If there is no task running, returns `Err(CopyResponseErr::NotReady)` if a task is pending and a
/// response is expected in the future, or `Err(CopyResponseErr::NotRunning)` if no task is running.
pub fn copy_response(dst: &mut [u8]) -> Result<usize, CopyResponseErr> {
    let mut state = USB_TASK_STATE.0.borrow_mut();
    match *state {
        UsbTaskState::Nothing => Err(CopyResponseErr::NotRunning),
        UsbTaskState::Running(Some(_), ref mut next_request_state) => {
            if let WaitingForNextRequestState::SendingResponse(ref response) = next_request_state {
                let len = response.len();
                dst[..len].copy_from_slice(response);
                *next_request_state = WaitingForNextRequestState::AwaitingRequest;
                Ok(len)
            } else {
                Err(CopyResponseErr::NotReady)
            }
        }
        UsbTaskState::Running(_, _) => Err(CopyResponseErr::NotReady),
        UsbTaskState::ResultAvailable(ref response) => {
            let len = response.len();
            dst[..len].copy_from_slice(response);
            *state = UsbTaskState::Nothing;
            Ok(len)
        }
    }
}

/// Cancel and drop a running task. Returns true if a task was cancelled, false if no task was
/// running.
pub fn cancel() -> bool {
    let mut state = USB_TASK_STATE.0.borrow_mut();
    if let UsbTaskState::Running(_, _) = *state {
        *state = UsbTaskState::Nothing;
        return true;
    }
    false
}

/// Must be called during the execution of a usb task. This sends out the response to the host and
/// awaits the next request.
pub async fn next_request(response: UsbOut) -> UsbIn {
    // Scope so that `state` is dropped before `.await`, see
    // https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_refcell_ref
    {
        let mut state = USB_TASK_STATE.0.borrow_mut();
        match *state {
            UsbTaskState::Running(None, ref mut next_request_state) => {
                *next_request_state = WaitingForNextRequestState::SendingResponse(response);
            }
            _ => panic!("next_request() called in wrong state"),
        }
    }

    option(&NEXT_REQUEST.0).await
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::prelude::v1::*;

    fn assert_panics<F: FnOnce() + std::panic::UnwindSafe>(f: F) {
        assert!(std::panic::catch_unwind(f).is_err());
    }

    /// Test spawning a task, spinning it, and getting the result.
    #[test]
    fn test_full_cycle() {
        async fn task(usb_in: UsbIn) -> UsbOut {
            assert_eq!(usb_in, [1, 2, 3].to_vec());
            [4, 5, 6, 7].to_vec()
        }
        fn assert_spawn_fails() {
            assert_panics(|| spawn(task, &[1, 2, 3]));
        }
        // repeated task processing ok
        for _ in 0..3 {
            let mut response = [0; 100];

            // No task running, can't copy response.
            assert_eq!(
                Err(CopyResponseErr::NotRunning),
                copy_response(&mut response)
            );

            spawn(task, &[1, 2, 3]);

            // Can't spawn: task already running.
            assert_spawn_fails();

            // Task not complete, can't copy response.
            assert_eq!(Err(CopyResponseErr::NotReady), copy_response(&mut response));

            spin();

            // Can't spawn: result not fetched yet
            assert_spawn_fails();

            // Response buffer too short.
            assert_panics(move || {
                let _ = copy_response(&mut response[..1]);
            });
            assert_eq!(Ok(4), copy_response(&mut response));
            // Response ok.
            assert_eq!(&response[..4], &[4, 5, 6, 7]);
        }
    }

    #[test]
    fn test_next_request() {
        async fn task(usb_in: UsbIn) -> UsbOut {
            assert_eq!(&usb_in, &[1, 2, 3]);
            let next_req = next_request([4, 5, 6, 7].to_vec()).await;
            assert_eq!(&next_req, &[8, 9, 10]);

            let next_req = next_request([11, 12].to_vec()).await;
            assert_eq!(&next_req, &[13, 14]);
            [15, 16, 17].to_vec()
        }

        let mut response = [0; 100];

        spawn(task, &[1, 2, 3]);
        spin();
        // Intermediate response.
        assert_eq!(Ok(4), copy_response(&mut response));
        assert_eq!(&response[..4], &[4, 5, 6, 7]);

        // Send follow-up request.
        assert!(waiting_for_next_request());
        on_next_request(&[8, 9, 10]);
        spin();

        // Intermediate response.
        assert_eq!(Ok(2), copy_response(&mut response));
        assert_eq!(&response[..2], &[11, 12]);

        // Send follow-up request.
        assert!(waiting_for_next_request());
        on_next_request(&[13, 14]);
        spin();

        // Final response.
        assert_eq!(Ok(3), copy_response(&mut response));
        assert_eq!(&response[..3], &[15, 16, 17]);
    }
}
