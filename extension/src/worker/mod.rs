mod task;
use crossbeam_deque::{Steal, Worker};
pub use task::{fn_task, Task, TaskExecutor};

pub fn init() -> Worker<Task> {
    Worker::<Task>::new_fifo()
}

pub fn start() -> Worker<Task> {
    let queue = init();
    for _ in 0..(std::env::var("DYNULO_THREADS")
        .unwrap_or_else(|_| "4".to_string())
        .parse::<i32>()
        .unwrap_or(4))
    {
        let stealer = queue.stealer();
        std::thread::spawn(move || loop {
            if let Steal::Success(task) = stealer.steal() {
                if let Err(e) = task.execute() {
                    error!("Error in task {}: {}", task.id(), e);
                }
            }
        });
    }
    queue
}
