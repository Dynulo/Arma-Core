mod task;
use crossbeam_deque::{Steal, Worker};
pub use task::{fn_task, Task, TaskExecutor};

pub fn init() -> Worker<Task> {
    Worker::<Task>::new_fifo()
}

pub fn start() -> Worker<Task> {
    let queue = init();
    let stealer = queue.stealer();
    std::thread::spawn(move || loop {
        if let Steal::Success(task) = stealer.steal() {
            if let Err(e) = task.execute() {
                error!("Error in task {}: {}", task.id(), e);
            }
        }
    });
    queue
}
