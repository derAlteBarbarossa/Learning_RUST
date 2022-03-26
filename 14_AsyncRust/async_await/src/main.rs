#![allow(dead_code)]
#![allow(unused_variables)]

use std::{thread, time};

use futures::{
    executor::block_on,
    join,
    select,
    future::FutureExt,
    pin_mut,
};

async fn print_message(string: &str) {
    println!("Async World: {}", string);
}

async fn task(string: &str, task_num: u8) {
    println!("Before starting async task {}", task_num);

    thread::sleep(time::Duration::from_secs(task_num.into()));
    print_message(string).await;

    println!("After Finishing async task {}", task_num);

}

async fn async_main_await() {
    task("task 1 is running", 1).await;
    task("task 2 is running", 2).await;
}

async fn async_main_join() {
    let task1 = task("task 1 is running", 1);
    let task2 = task("task 2 is running", 2);

    join!(task1, task2);
}

async fn async_main_select() {
    // fuse() converts the future to FusedFuture
    // FusedFuture signals the select! macro not to
    // poll for a completed future

    let task1 = task("task 1 is running", 1).fuse();
    let task2 = task("task 2 is running", 2).fuse();

    // pin_mut! macro converts the tasks to references
    // In this way, ownership of tasks is not taken and
    // uncompleted tasks can be polled again
    pin_mut!(task1, task2);

    loop {
        select! {
            () = task1 => println!("task 1 finished first"),
            () = task2 => println!("task 2 finished first"),
            complete => break,
        }

    }
}
fn main() {
    // block_on(async_main_await());
    // block_on(async_main_join());
    // block_on(async_main_select());
}
