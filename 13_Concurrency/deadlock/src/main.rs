use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(1));

    let mut handles = vec![];

    {
        let lock1 = Arc::clone(&a);
        let lock2 = Arc::clone(&b);
   
        let handle = thread::spawn(move|| {
            let mut a = lock1.lock().unwrap();
            *a += 1;

            thread::sleep(Duration::from_secs(1));

            let mut b = lock2.lock().unwrap();
            *b += 1;
        });
        handles.push(handle);
    }

    {
        let lock1 = Arc::clone(&a);
        let lock2 = Arc::clone(&b);

        let handle = thread::spawn(move|| {
            let mut b = lock2.lock().unwrap();
            *b += 1;

            thread::sleep(Duration::from_secs(1));

            let mut a = lock1.lock().unwrap();
            *a += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finished both threads!");
}
