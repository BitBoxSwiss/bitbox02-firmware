// SPDX-License-Identifier: Apache-2.0

//! This module provides the executor for tasks that are spawned with an API request and deliver a
//! USB response. Terminology: host = computer, device = BitBox02.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use util::futures::completion;

type UsbOut = Vec<u8>;
type UsbIn = Vec<u8>;
type UsbTask = Pin<Box<dyn Future<Output = UsbOut> + 'static>>;

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
    SendingResponse(UsbOut, completion::Responder<UsbIn>),
    /// Host got the response, now we are waiting for the next request by the host.
    AwaitingRequest(completion::Responder<UsbIn>),
}

/// Describes the global state of an api query. The documentation of
/// the variants apply to the HWW stack, but have analogous meaning in
/// the U2F stack.
enum UsbTaskState {
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
    Running(Option<UsbTask>, WaitingForNextRequestState),
    /// The task has finished and written the result, so the USB response is available. We are now
    /// waiting for the host to fetch it (HWW_REQ_RETRY). For short-circuited or non-async api
    /// calls, the result might be returned immediately in response to HWW_REQ_NEW.
    ResultAvailable(UsbOut),
}

/// A safer version of UsbTaskState. RefCell so we cannot accidentally borrow illegally.
struct SafeUsbTaskState(RefCell<UsbTaskState>);

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
            let task: UsbTask = Box::pin(workflow(usb_in.to_vec()));

            *state = UsbTaskState::Running(Some(task), WaitingForNextRequestState::Idle);
        }
        // This panic could happen e.g. if someone reconnects to the BitBox while a task is running,
        // before the 500ms timeout cancels the task. The proper way to handle would be to let the
        // host know we are busy so the host can re-retry after some time.
        _ => panic!("spawn: wrong state"),
    }
}

/// Returns true if a request is being processed and waiting for another request via the
/// `next_request()` future.
pub fn waiting_for_next_request() -> bool {
    matches!(
        *USB_TASK_STATE.0.borrow(),
        UsbTaskState::Running(Some(_), WaitingForNextRequestState::AwaitingRequest(_))
    )
}

pub fn is_idle() -> bool {
    matches!(*USB_TASK_STATE.0.borrow(), UsbTaskState::Nothing)
}

/// Resolves the `next_request()` future. `waiting_for_next_request()` must be true when calling
/// this, otherwise this function panics.
pub fn on_next_request(usb_in: &[u8]) {
    let mut state = USB_TASK_STATE.0.borrow_mut();
    match &mut *state {
        UsbTaskState::Running(
            Some(_),
            next_request_state @ WaitingForNextRequestState::AwaitingRequest(_),
        ) => {
            let WaitingForNextRequestState::AwaitingRequest(responder) = next_request_state else {
                unreachable!();
            };
            responder.resolve(usb_in.to_vec());
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
        let context = &mut Context::from_waker(Waker::noop());
        let spin_result = task.as_mut().poll(context);
        if matches!(*USB_TASK_STATE.0.borrow(), UsbTaskState::Nothing) {
            // The task was cancelled while it was running, so there is nothing to do with the
            // result.
            return;
        }
        match spin_result {
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
/// If a result is available (state = `ResultAvailable`), this returns the usb response and moves
/// the state to `Nothing`.
///
/// If a task is pending and a response is expected in the future, returns
/// `Err(CopyResponseErr::NotReady)`. If no task is running, returns
/// `Err(CopyResponseErr::NotRunning)`.
pub fn take_response() -> Result<UsbOut, CopyResponseErr> {
    let mut state = USB_TASK_STATE.0.borrow_mut();
    match &mut *state {
        UsbTaskState::Nothing => Err(CopyResponseErr::NotRunning),
        UsbTaskState::Running(Some(_), next_request_state) => {
            match core::mem::replace(next_request_state, WaitingForNextRequestState::Idle) {
                WaitingForNextRequestState::SendingResponse(response, responder) => {
                    *next_request_state = WaitingForNextRequestState::AwaitingRequest(responder);
                    Ok(response)
                }
                next_request_state_value => {
                    *next_request_state = next_request_state_value;
                    Err(CopyResponseErr::NotReady)
                }
            }
        }
        UsbTaskState::Running(_, _) => Err(CopyResponseErr::NotReady),
        UsbTaskState::ResultAvailable(response) => {
            let response = core::mem::take(response);
            *state = UsbTaskState::Nothing;
            Ok(response)
        }
    }
}

/// Reset all outstanding USB task state.
///
/// This drops a running task, any unread final response, and any pending `next_request()` input.
/// It is used when the host disappears or when the transport times out waiting for the host to
/// fetch a response. Call this inside a running task only if you expect that the host may not be
/// able to read the result (e.g. when resetting the BLE chip as part of a task), so another task
/// can spawn afterwards immediately instead of being blocked by stale executor state.
pub fn cancel() {
    let mut state = USB_TASK_STATE.0.borrow_mut();
    *state = UsbTaskState::Nothing;
}

/// Must be called during the execution of a usb task. This sends out the response to the host and
/// awaits the next request.
pub async fn next_request(response: UsbOut) -> UsbIn {
    // Scope so that `state` is dropped before `.await`, see
    // https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_refcell_ref
    let result = {
        let mut state = USB_TASK_STATE.0.borrow_mut();
        let (responder, result) = completion::completion();
        match *state {
            UsbTaskState::Running(None, ref mut next_request_state) => {
                *next_request_state =
                    WaitingForNextRequestState::SendingResponse(response, responder);
            }
            _ => panic!("next_request() called in wrong state"),
        }
        result
    };
    result.await
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::prelude::v1::*;
    use std::sync::{Mutex, MutexGuard};

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn test_guard() -> MutexGuard<'static, ()> {
        let guard = TEST_LOCK.lock().unwrap();
        cancel();
        guard
    }

    fn assert_panics<F: FnOnce() + std::panic::UnwindSafe>(f: F) {
        assert!(std::panic::catch_unwind(f).is_err());
    }

    /// Test spawning a task, spinning it, and getting the result.
    #[test]
    fn test_full_cycle() {
        let _guard = test_guard();

        async fn task(usb_in: UsbIn) -> UsbOut {
            assert_eq!(usb_in, [1, 2, 3].to_vec());
            [4, 5, 6, 7].to_vec()
        }
        fn assert_spawn_fails() {
            assert_panics(|| spawn(task, &[1, 2, 3]));
        }
        // repeated task processing ok
        for _ in 0..3 {
            // No task running, can't take response.
            assert_eq!(Err(CopyResponseErr::NotRunning), take_response());

            spawn(task, &[1, 2, 3]);

            // Can't spawn: task already running.
            assert_spawn_fails();

            // Task not complete, can't take response.
            assert_eq!(Err(CopyResponseErr::NotReady), take_response());

            spin();

            // Can't spawn: result not fetched yet
            assert_spawn_fails();

            // Response ok.
            assert_eq!(Ok(vec![4, 5, 6, 7]), take_response());
        }
    }

    #[test]
    fn test_next_request() {
        let _guard = test_guard();

        async fn task(usb_in: UsbIn) -> UsbOut {
            assert_eq!(&usb_in, &[1, 2, 3]);
            let next_req = next_request([4, 5, 6, 7].to_vec()).await;
            assert_eq!(&next_req, &[8, 9, 10]);

            let next_req = next_request([11, 12].to_vec()).await;
            assert_eq!(&next_req, &[13, 14]);
            [15, 16, 17].to_vec()
        }

        spawn(task, &[1, 2, 3]);
        spin();
        // Intermediate response.
        assert_eq!(Ok(vec![4, 5, 6, 7]), take_response());

        // Send follow-up request.
        assert!(waiting_for_next_request());
        on_next_request(&[8, 9, 10]);
        spin();

        // Intermediate response.
        assert_eq!(Ok(vec![11, 12]), take_response());

        // Send follow-up request.
        assert!(waiting_for_next_request());
        on_next_request(&[13, 14]);
        spin();

        // Final response.
        assert_eq!(Ok(vec![15, 16, 17]), take_response());
    }

    #[test]
    fn test_take_response() {
        let _guard = test_guard();

        async fn task(_usb_in: UsbIn) -> UsbOut {
            [4, 5, 6, 7].to_vec()
        }

        spawn(task, &[1, 2, 3]);
        spin();

        assert_eq!(Ok(vec![4, 5, 6, 7]), take_response());
        assert!(is_idle());
    }

    #[test]
    fn test_take_response_waiting_for_next_request() {
        let _guard = test_guard();

        async fn task(_usb_in: UsbIn) -> UsbOut {
            let next_req = next_request([4, 5, 6, 7].to_vec()).await;
            assert_eq!(&next_req, &[8, 9, 10]);
            [11, 12].to_vec()
        }

        spawn(task, &[1, 2, 3]);
        spin();

        assert_eq!(Ok(vec![4, 5, 6, 7]), take_response());
        assert!(waiting_for_next_request());

        on_next_request(&[8, 9, 10]);
        spin();

        assert_eq!(Ok(vec![11, 12]), take_response());
        assert!(is_idle());
    }

    #[test]
    fn test_cancel_clears_result_available() {
        let _guard = test_guard();

        async fn task(_usb_in: UsbIn) -> UsbOut {
            [4, 5, 6, 7].to_vec()
        }

        spawn(task, &[1, 2, 3]);
        spin();
        assert!(!is_idle());

        cancel();
        assert!(is_idle());
        assert_eq!(Err(CopyResponseErr::NotRunning), take_response());

        spawn(task, &[1, 2, 3]);
        spin();
        assert_eq!(Ok(vec![4, 5, 6, 7]), take_response());
    }

    #[test]
    fn test_cancel_clears_pending_next_request() {
        let _guard = test_guard();

        async fn first_task(_usb_in: UsbIn) -> UsbOut {
            let next_req = next_request([1, 2].to_vec()).await;
            assert_eq!(&next_req, &[3, 4]);
            [5, 6].to_vec()
        }

        async fn second_task(_usb_in: UsbIn) -> UsbOut {
            let next_req = next_request([7].to_vec()).await;
            assert_eq!(&next_req, &[9]);
            [8].to_vec()
        }

        spawn(first_task, &[]);
        spin();
        assert_eq!(Ok(vec![1, 2]), take_response());
        assert!(waiting_for_next_request());

        on_next_request(&[3, 4]);
        cancel();
        assert!(is_idle());

        spawn(second_task, &[]);
        spin();
        assert_eq!(Ok(vec![7]), take_response());
        assert!(waiting_for_next_request());

        on_next_request(&[9]);
        spin();
        assert_eq!(Ok(vec![8]), take_response());
    }
}
