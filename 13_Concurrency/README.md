# Concurrency



Withing a *process* we can split the jobs to some **independent** parts running simultaneously. This parts are called **thread**s. As threads run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. In a non-safe threading, we could encounter different issues like:
1. Data Race
2. Deadlock
3. Bugs, which reveal themselves only under some certain circumstances


> Initially, the Rust team thought that ensuring memory safety and pre- venting concurrency problems were two separate challenges to be solved with different methods. Over time, the team discovered that the owner- ship and type systems are a powerful set of tools to help manage memory safety and concurrency problems!
> -- <cite>The Rust Programming Language, S. Klabnik & C. Nochols</cite>

As your experience tells you, debugging a concurrent program is unimaginable hard. Many runtime  erros happen only under very special circumstances and reproducing them is a nightmare. Rust really eases concurrent programming by leveraging those already mentioned ownership rules. Hence, instead of being surprised by some very rare occuring runtime errors, you would get a compile-time error.

Threading would be possible, when we have an API exposed by the underlying OS to the runtime. Rust threading model is **1:1**, which means evey userspace thread is mapped to one OS thread. This model has a great advantage with one of the crux philosophies behind Rust: small runtime. Because, there's no runtime implementation for mapping M userspace threads to N OS threads. 

## Creating Threads

To create a new thread, we call `thread::spawn` and pass it a closure. This function returns a `Handle` to that spawned thread for further accesses.

```rust
use std::thread;
use std::time::Duration;
fn main() {
	let handle = thread::spawn(|| {
        for i in 1..10 {
			println!("hi number {} from the spawned thread!", i);
			thread::sleep(Duration::from_millis(1));
		}
	});
    
	for i in 1..5 {
		println!("hi number {} from the main thread!", i);
		thread::sleep(Duration::from_millis(1));
	}
}
```

In the code above, we have to threads, one `main` thread and one with `handle`. If you run it multiple times, you can see that mose of the runs end up with an uncomplete `handle` thread execution. As two threads run independently, if the `main` thread finishes earlier, the `handle` would instantly be terminated.

If we want to wait for the spawned thread to finish its execution, we need to *block* the main thread:

```rust
use std::thread;
use std::time::Duration;
fn main() {
	let handle = thread::spawn(|| {
        for i in 1..10 {
			println!("hi number {} from the spawned thread!", i);
			thread::sleep(Duration::from_millis(1));
		}
	});
    
	for i in 1..5 {
		println!("hi number {} from the main thread!", i);
		thread::sleep(Duration::from_millis(1));
	}

	handle.join().unwrap();
}
```

### Move Closure

When inside our closure, we want to access environment values, we need to move them into the scope of the spawned thread. This `move` guarantees that multiple threads would not try to access that value in an uncontrolled manner.

```rust
use std::thread;
fn main() {
	let v = vec![1, 2, 3];
	let handle = thread::spawn(move || {
		println!("Here's a vector: {:?}", v);
});
	handle.join().unwrap();
}
```
## Message Passing between Threads

For message passing, Rust has implemented a thread-safe data structure, named **Channel**. Like any communication channel, Rust channels also have two endpoints, Receiver and Transmitter. Rust channel is a so-called Mutiple Producer, Single Consumer **mpsc** means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.

```rust
use std::thread;
use std::sync::mpsc;
fn main() {
	let (tx, rx) = mpsc::channel();
	thread::spawn(move || {
		let val = String::from("hi");
		tx.send(val).unwrap();
	});
	let received = rx.recv().unwrap();
	println!("Got: {}", received);
}

```

To send a value we call `send` method in the transmitter endpoint. It reaturns a `Result<T, E>`. So, if the receiving point is already dropped, we get an error.

On the receiving endpoint, we can either receive data blocking or non-blocking:
1. `recv()` is blocking, which will block the main thread’s execu- tion and wait until a value is sent down the channel. Once a value is sent from the other point, it returns a `Result<T, E>`
2. `try_recv()` is non-blocking and immidiately returns a `Result<T, E>`

### Owenship
Once a value has been sent down to a channel, the sender thread is no more the owner of that value. It makes fully sense. Just imagine that after senbding the value, meanwhile the receiving end is working on it, the transmitter decides to drop it. It would be an absolut mess.

### Sending/Receiving Multiple Values

`rx` can serve us as an iterator:

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
fn main() {
	let (tx, rx) = mpsc::channel();
	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
			];

		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in rx {
		println!("Got: {}", received);
	}
}

```

### Multiple Producers
To support multiple producers on a channel, we have to clone `tx` endpoint:

```rust
let (tx, rx) = mpsc::channel();
let tx1 = mpsc::Sender::clone(&tx);
```

## Shared-State Concurreny
Channels are like a single ownership, as when you transfer a value to the other end, you can no longer have access to it, unless the other endpoint sends it back. This, however, imposes a high overhead on the system in terms of overhead.

**Shared Memory** concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.

To make shared memory cuncurrency possible, we need two primitives:
1. Mutual Exclusion: Like multithreading in other languages, we need to make sure that at any given time, only one thread can access a value.
2. Multiple Ownership: This is specific to Rust, as by default we can't have multiple ownership.

In what follows, we talk about how to satisfy these two requirements.

### Mutual Exclusion
Mutual Exclusioln, or abbreviated as **Mutex** allows only one thread to access a value at any given time. To access the data wrapped in a mutex, a thread must first announce that it wants access and acquire the mutex’s **lock**. Lock is a data structure that keeps track of who currently has exclusive access to the data.

```rust
use std::sync::Mutex;

fn main() {
	let m = Mutex::new(5); {
	let mut num = m.lock().unwrap();
	*num = 6;
	}

	println!("m = {:?}", m);
}
```
### Multiple Ownership of Mutex
Very similar to **Rc<T>**, we have an Atomic Reference Counting, **Arc<T>**, which supports multiple ownership in a thread-safe manner.

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];

	for _ in 0..10 {
		let counter = Arc::clone(&counter);
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;
		});
        
		handles.push(handle);
    }

	for handle in handles {
		handle.join().unwrap();
	}

	println!("Result: {}", *counter.lock().unwrap());

}

```



















