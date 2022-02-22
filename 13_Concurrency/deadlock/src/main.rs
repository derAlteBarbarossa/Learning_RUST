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
  
        // Execution flow is annotated with numbers:

        // 1. First thread is spawned. It starts executing
        // 2. while main thread pushes its handle to `handles`.
        // 3. It acquires a lock for a.
        // 4. Then, it goes to sleep for 1 second.
        // 9 (deadlock): it tries to acquire a lock for `b`,
        // which is being held by the second thread.
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

        // 5. Second thread is spawned. It starts executing
        // 6. while main thread pushes its handle to `handles`.
        // 7. It acquires a lock for `b`.
        // 8. Then, it goes to sleep for 1 second.
        // 9 (deadlock): it tries to acquire a lock for `a`,
        // which is being held by the first thread.

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
