use std::net::UdpSocket;

fn main()
{
    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("Unable to bind to a socket");

    socket.connect("127.0.0.1:3000")
        .expect("Unable to bind to server");

    println!("Socker peer address is: {:?}", socket.peer_addr());

    socket.send("\"Hello\" using send() system".as_bytes())
        .expect("Unable to send data");


}