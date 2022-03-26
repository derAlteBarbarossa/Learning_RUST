#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    thread,
    time,
    sync::{
        Mutex,
        Arc,
    }, 
};

use futures::{
    executor::block_on,
    join,
    select,
    future::FutureExt,
    pin_mut,
};

struct shared_log {
    logs: String
}

async fn print_message(string: &str) {
    println!("Async World: {}", string);
}

async fn task(string: &str, task_num: u8, log: &Arc<Mutex<shared_log>>) {
    println!("Before starting async task {}", task_num);

    thread::sleep(time::Duration::from_secs(task_num.into()));
    print_message(string).await;

    println!("After Finishing async task {}", task_num);

    if let Ok(mut log) = log.lock() {
        log.logs = format!("{}{}", log.logs, string);
    }
}

async fn async_main_await(log: &Arc<Mutex<shared_log>>) {
    // Rust Future's are lazy. By calling `await` on them,
    // we schedule them to be run
    task("task 1 is running", 1, &log).await;
    task("task 2 is running", 2, &log).await;
}

async fn async_main_join(log: &Arc<Mutex<shared_log>>) {
    let task1 = task("task 1 is running", 1, &log);
    let task2 = task("task 2 is running", 2, &log);

    join!(task1, task2);
}

async fn async_main_select(log: &Arc<Mutex<shared_log>>) {
    // fuse() converts the future to FusedFuture
    // FusedFuture signals the select! macro not to
    // poll for a completed future

    let task1 = task("task 1 is running", 1, &log).fuse();
    let task2 = task("task 2 is running", 2, &log).fuse();

    // pin_mut! macro converts the tasks to references
    // In this way, ownership of tasks is not taken and
    // uncompleted tasks can be polled again
    pin_mut!(task1, task2);

    loop {
        select! {
            () = task1 => {}, 
            () = task2 => {}, 
            complete => break,
        }

    }
}
fn main() {
    let mut log = Arc::new(Mutex::new(shared_log {
        logs: "".to_string(),
    })); 
   
    println!("Running with await...");
    block_on(async_main_await(&log));
    println!("---------------------------");
    
    println!("Running with join...");
    block_on(async_main_join(&log));
    println!("---------------------------");

    println!("Running with select...");
    block_on(async_main_select(&log));
    println!("---------------------------");
}
