use std::{
    fs::File,
    io::{Read, Write},
    sync::{Mutex, RwLock},
};

use arma_rs::{rv, rv_handler};
use crossbeam_deque::Worker;

#[macro_use]
extern crate log;

mod logger;
mod task;
mod worker;

use task::Task;

lazy_static::lazy_static! {
    static ref HOST: String = std::env::var("DYNULO_HOST")
        // .unwrap_or_else(|_| "https://commander.dynulo.com/".to_string());
        .unwrap_or_else(|_| "http://localhost:8000/".to_string());
    static ref TOKEN: RwLock<String> = RwLock::new(String::new());
    static ref QUEUE: Mutex<Worker<Task>> = Mutex::new(Worker::<Task>::new_fifo());
}

#[rv]
fn register_token(token: String) {
    *TOKEN.write().unwrap() = token.clone();
    let mut file = File::create("token.txt").unwrap();
    file.write_all(token.as_bytes()).unwrap();
    info!("token saved to file");
}

#[rv]
fn has_token() -> bool {
    !TOKEN.read().unwrap().is_empty()
}

#[rv]
fn uuid() -> String {
    ::uuid::Uuid::new_v4().to_string()
}

#[rv]
fn apicall(id: String, method: String, path: String, body: String) {
    if let Ok(q) = QUEUE.lock() {
        q.push(Task::apicall(id, method, path, body));
    } else {
        error!("failed to lock queue");
    }
}

#[rv_handler]
fn init() {
    logger::init();

    if let Ok(mut file) = File::open("token.txt") {
        let mut token = String::new();
        file.read_to_string(&mut token).unwrap();
        *TOKEN.write().unwrap() = token;
        info!("token loaded from file");
    }

    worker::init();
}
