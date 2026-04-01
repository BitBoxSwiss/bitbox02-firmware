// SPDX-License-Identifier: Apache-2.0

use alloc::rc::Rc;
use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

struct SharedState<T> {
    waker: Option<Waker>,
    result: Option<T>,
}

pub struct Responder<T> {
    shared_state: Rc<RefCell<SharedState<T>>>,
}

pub struct Result<T> {
    shared_state: Rc<RefCell<SharedState<T>>>,
}

pub fn completion<T>() -> (Responder<T>, Result<T>) {
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    (
        Responder {
            shared_state: Rc::clone(&shared_state),
        },
        Result { shared_state },
    )
}

impl<T> Responder<T> {
    pub fn resolve(&self, value: T) {
        let mut shared_state = self.shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(value);
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }
}

impl<T> Clone for Responder<T> {
    fn clone(&self) -> Self {
        Self {
            shared_state: Rc::clone(&self.shared_state),
        }
    }
}

impl<T> Future for Result<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.borrow_mut();
        if let Some(result) = shared_state.result.take() {
            Poll::Ready(result)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use core::pin::pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Wake;

    struct CountingWake {
        wake_count: AtomicUsize,
    }

    impl CountingWake {
        fn new() -> Self {
            Self {
                wake_count: AtomicUsize::new(0),
            }
        }

        fn wake_count(&self) -> usize {
            self.wake_count.load(Ordering::SeqCst)
        }
    }

    impl Wake for CountingWake {
        fn wake(self: Arc<Self>) {
            self.wake_count.fetch_add(1, Ordering::SeqCst);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.wake_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_completion_resolve_before_poll() {
        let (responder, result) = completion();
        responder.resolve(42);

        let waker = std::task::Waker::from(Arc::new(CountingWake::new()));
        let mut cx = Context::from_waker(&waker);
        let mut result = pin!(result);

        assert!(matches!(result.as_mut().poll(&mut cx), Poll::Ready(42)));
    }

    #[test]
    fn test_completion_resolve_wakes() {
        let (responder, result) = completion();
        let wake = Arc::new(CountingWake::new());
        let waker = std::task::Waker::from(Arc::clone(&wake));
        let mut cx = Context::from_waker(&waker);
        let mut result = pin!(result);

        assert!(matches!(result.as_mut().poll(&mut cx), Poll::Pending));
        assert_eq!(wake.wake_count(), 0);

        responder.resolve(42);
        assert_eq!(wake.wake_count(), 1);
        assert!(matches!(result.as_mut().poll(&mut cx), Poll::Ready(42)));
    }

    #[test]
    fn test_completion_first_resolution_wins() {
        let (responder, result) = completion();
        responder.resolve(42);
        responder.resolve(99);

        let waker = std::task::Waker::from(Arc::new(CountingWake::new()));
        let mut cx = Context::from_waker(&waker);
        let mut result = pin!(result);

        assert!(matches!(result.as_mut().poll(&mut cx), Poll::Ready(42)));
    }
}
