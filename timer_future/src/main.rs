use std::{sync::{Arc, mpsc::{SyncSender, sync_channel, Receiver}, Mutex}, task::Context, time::Duration};

use futures::{future::BoxFuture, Future, FutureExt, task::{ArcWake, waker_ref}};
use timer_future::TimerFuture;

struct Executor{
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Debug)]
struct Spawner{
    task_sender: SyncSender<Arc<Task>>,
}

struct Task{
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner()->(Executor, Spawner){
    const MAX_QUEUED_TASKS: usize  = 10_000;

    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    (Executor{ready_queue}, Spawner{task_sender})
}

impl Spawner{
    fn spawn(&self, future: impl Future<Output=()> + 'static + Send){
        let future = future.boxed();
        let task = Arc::new(Task{
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");

    }
}

impl ArcWake for Task{
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();

        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take(){
                let waker  = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending(){
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main(){
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async{
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    drop(spawner);

    executor.run();
}