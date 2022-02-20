use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main()
{
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    let msg_to_send = "Hello from TCP client".as_bytes();

    stream.write(msg_to_send).unwrap();

    let mut buffer = [0; 100];

    stream.read(&mut buffer).unwrap();

    println!("Server echoed: {:?}",
        str::from_utf8(&buffer)
        .unwrap()
        .trim_end_matches(char::from(0)));
}