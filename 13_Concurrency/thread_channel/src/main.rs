use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let tx1 = mpsc::Sender::clone(&tx);
    
    thread::spawn(move || {
        let vals = vec![ String::from("hi"), 
                        String::from("from"), 
                        String::from("the"), 
                        String::from("first"),
                        String::from("thread"),
                    ];

        for val in vals {
            tx.send(val).unwrap(); 
        }
    });

    thread::spawn(move || {
        let vals = vec![ String::from("مرحبا"), 
                        String::from("من"), 
                        String::from("الخیط"), 
                        String::from("الثاني"),
                    ];

        for val in vals {
            tx1.send(val).unwrap(); 
        }
    });

    // Blocking!
    for received in rx { 
        println!("Got: {}", received);
    }
}