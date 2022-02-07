#![allow(dead_code)]

use std::process;
use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::panic;


fn exit_custom(code: i32)
{
    println!("Going to exit the process with code {}", code);
    process::exit(code);
}

fn check_status(file_name: &str)
{
    let status = Command::new("cat")
                // .stdout(Stdio::null())
                .arg(file_name)
                .status()
                .expect("failed to excute cat");

    if status.success() {
        println!("Successful Operation");
    } else {
        println!("Unsuccessful Operation");
    }
}

fn read_child_stdout()
{
    let process = match Command::new("ps").stdout(Stdio::piped()).spawn() {
        Err(err) => panic!("Couldn't spawn ps: {}", err),
        Ok(process) => process,
    };

    let mut ps_output = String::new();

    match process.stdout.unwrap().read_to_string(&mut ps_output) {
        Err(err) => panic!("Couldn't read ps stdout: {}", err),
        Ok(_) => print!("ps output from child process is \n{}", ps_output),
    }

}

fn write_child_stdin()
{
    let process = match Command::new("rev")
                        .stdout(Stdio::piped())
                        .stdin(Stdio::piped())
                        .spawn()
    {
        Err(err) => panic!("Couldn't spawn rev: {}", err),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all("palindrome".as_bytes()) {
        Err(err) => panic!("Couldn't write to stdin: {}", err),
        Ok(_) => println!("Sent text to rev command"),
    }

    let mut child_output = String::new();
    match process.stdout.unwrap().read_to_string(&mut child_output) {
        Err(err) => panic!("Couldn't read from child's output: {}", err),
        Ok(_) => println!("Child replied: {}", child_output),
    }

}

fn set_env()
{
    Command::new("env")
    //.env_clear()
    .env("MY_PATH", "/tmp")
    //.env_remove("MY_PATH")
    .spawn()
    .expect("Command failed to execute");
}

fn custom_panic_hook()
{
    panic::set_hook(Box::new(|_| {
        println!("Here the custom panic hook is called!")
    }));

    let _child_process = match Command::new("invalid-command")
            .spawn() {
        Err(err) => panic!("Normal panic message {}", err),
        Ok(new_process_handle) => new_process_handle,
    };
                        
}

fn main() {
    // exit_custom(64);
    // check_status("src/main.rs");
    // read_child_stdout();
    // write_child_stdin();
    // set_env();
    custom_panic_hook();
}
