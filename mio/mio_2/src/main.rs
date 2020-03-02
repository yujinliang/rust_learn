use mio::net::UdpSocket;
use mio::{Events, Ready, Poll, PollOpt, Token};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {



    const SENDER: Token = Token(0);
    const ECHOER: Token = Token(1);

    // This operation will fail if the address is in use, so we select different ports for each
    // socket.
    let sender_socket = UdpSocket::bind(&"127.0.0.1:0".parse()?)?;
    let echoer_socket = UdpSocket::bind(&"127.0.0.1:0".parse()?)?;

    // If we do not use connect here, SENDER and ECHOER would need to call send_to and recv_from
    // respectively.
    sender_socket.connect(echoer_socket.local_addr().unwrap())?;

    // We need a Poll to check if SENDER is ready to be written into, and if ECHOER is ready to be
    // read from.
    let poll = Poll::new()?;

    // We register our sockets here so that we can check if they are ready to be written/read.
    poll.register(&sender_socket, SENDER, Ready::writable(), PollOpt::edge())?;
    poll.register(&echoer_socket, ECHOER, Ready::readable(), PollOpt::edge())?;

    let msg_to_send = [9; 9];
    let mut buffer = [0; 9];

    let mut events = Events::with_capacity(128);
    loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;
        for event in events.iter() {
            match event.token() {
                // Our SENDER is ready to be written into.
                SENDER => {
                    let bytes_sent = sender_socket.send(&msg_to_send)?;
                    assert_eq!(bytes_sent, 9);
                    println!("sent {:?} -> {:?} bytes", msg_to_send, bytes_sent);
                },
                // Our ECHOER is ready to be read from.
                ECHOER => {
                    let num_recv = echoer_socket.recv(&mut buffer)?;
                    println!("echo {:?} -> {:?}", buffer, num_recv);
                    buffer = [0; 9];
                }
                _ => unreachable!()
            }
        }
    } //loop end.
}
