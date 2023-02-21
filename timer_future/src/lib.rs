use std::{sync::{Mutex, Arc}, task::{Waker, Poll}, future::Future, time::Duration, thread};

pub struct TimerFuture{
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState{
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture{
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed{
            Poll::Ready(())
        }else{
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture{
    pub fn new(duration: Duration) -> Self{
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed: false,
            waker: None,
        }));
        
        let thread_shared_state = shared_state.clone();

        thread::spawn(move||{
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.completed = true;

            if let Some(waker) = shared_state.waker.take(){
                waker.wake()
            }
        });

        TimerFuture { shared_state: shared_state }
    }
}
