use std::sync::Mutex;

use arma_rs::{arma, Extension};
use crossbeam_deque::Worker;

use crate::worker::Task;

#[macro_use]
extern crate log;

mod commands;
mod logger;
mod token;
mod worker;

lazy_static::lazy_static! {
    static ref QUEUE: Mutex<Worker<Task>> = Mutex::new(Worker::<Task>::new_fifo());
}

#[arma]
fn init() -> Extension {
    commands::core::init();
    *QUEUE.lock().unwrap() = worker::start();
    let ext = Extension::build()
        .group("core", commands::core::group())
        .group("discord", commands::discord::group())
        .group("api", commands::api::group())
        .finish();
    logger::init(ext.context());
    ext
}
