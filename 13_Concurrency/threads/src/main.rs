#![allow(dead_code)]

use std::thread;

fn spawn_thread(num_thread: usize)
{
    for _ in 1..num_thread {
        thread::spawn(|| {
            println!("Hi from thread id {:?}", thread::current().id());
        });
    }
}

fn spawn_and_join(num_thread: usize)
{
    let mut child_threads: Vec<thread::JoinHandle<_>> = Vec::new();

    for _ in 1..num_thread {
        let handle: thread::JoinHandle<_> = thread::spawn(|| {
            println!("Hi from thread id {:?}", thread::current().id());
        });
        child_threads.push(handle);
    }

    for thread in child_threads {
        thread.join().unwrap();
    }
}

fn spawn_with_builder(num_thread: usize)
{
    let mut child_threads: Vec<thread::JoinHandle<_>> = Vec::new();

    for i in 1..num_thread {
        let builder: thread::Builder = thread::Builder::new().name(format!("mythread{}", i));

        let handle = builder.spawn(|| {
            println!("Hi from thread id {:?}", thread::current().id());
        });

        child_threads.push(handle.unwrap());
    }

    for thread in child_threads {
        thread.join().unwrap();
    }
}

fn main() {
    //spawn_thread(10);
    //spawn_and_join(10);
    spawn_with_builder(10);
}
