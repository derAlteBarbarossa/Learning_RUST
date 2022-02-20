#![allow(unused_variables)]
use std::fs::File;
use std::io::ErrorKind;

#[allow(dead_code)]
fn test_panic() {
    panic!("Burn and Crash!");
}

fn main() {
    // When an error is unrecoverable, we call `panic`
    // test_panic();

    // However, in some cases error is `recoverable` and
    // we can continue, probably with a reduced functionality
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

}
