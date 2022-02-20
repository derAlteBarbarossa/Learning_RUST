use std::str;
use std::thread;
use std::net::UdpSocket;

fn main() {

    let socket = UdpSocket::bind("127.0.0.1:3000")
        .expect("Unable to bind");
    let mut buffer = [0; 1024];

    loop {

        let new_socket = socket.try_clone()
            .expect("Unable to clone the socket");

        match new_socket.recv_from(&mut buffer) {
            Ok((num_bytes, origin_addr)) => {
                thread::spawn(move || {
                    println!("Received from client: {}", 
                        str::from_utf8(&buffer[..num_bytes])
                        .unwrap());
                
                    let response = format!("Received: {}",
                        String::from_utf8_lossy(&buffer[..num_bytes]));

                    new_socket.send_to(response.as_bytes(), origin_addr)
                    .unwrap();
                });
            },
            Err(err) =>
                println!("Error in receiving UDP: {}", err),
        }
    }
    
}
