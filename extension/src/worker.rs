use arma_rs::rv_callback;
use crossbeam_deque::{Steal, Worker};

use crate::{rv_send_callback, task::Task, HOST, QUEUE, TOKEN};

// TODO: look into using tokio instead of threads, but it's probably not worth it
pub fn init() {
    let num_threads = std::env::var("DYNULO_THREADS")
        .unwrap_or_else(|_| "2".to_string())
        .parse::<usize>()
        .unwrap_or(2);

    let queue = Worker::<Task>::new_fifo();

    for t in 0..num_threads {
        let stealer = queue.stealer();
        if let Err(e) = std::thread::Builder::new()
            .name(format!("dynulo_woker_{}", t))
            .spawn(move || {
                let client = reqwest::blocking::Client::new();
                loop {
                    if TOKEN.read().unwrap().is_empty() {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        continue;
                    }
                    if let Steal::Success(task) = stealer.steal() {
                        match task {
                            Task::ApiCall {
                                id,
                                path,
                                method,
                                body,
                            } => {
                                let path = format!("{}{}", *HOST, path);
                                let builder = match method.to_lowercase().as_str() {
                                    "get" => client.get(path),
                                    "post" => client.post(path),
                                    "put" => client.put(path),
                                    "delete" => client.delete(path),
                                    _ => panic!("Unsupported method: {}", method),
                                };
                                let response = builder
                                    .header("Authorization", TOKEN.read().unwrap().clone())
                                    .header("Content-Type", "application/json")
                                    .body(body)
                                    .send()
                                    .unwrap()
                                    .text()
                                    .unwrap();
                                rv_callback!("dynulo_core", "apicall", id, response);
                            }
                        }
                    }
                }
            })
        {
            error!("Failed to spawn worker thread: {}", e);
        } else {
            info!("Spawned worker thread {}", t);
        }
    }

    *QUEUE.lock().unwrap() = queue;
}
