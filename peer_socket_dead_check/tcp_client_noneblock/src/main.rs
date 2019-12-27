use mio::{Events, Ready, Poll, PollOpt, Token};
use mio::net::TcpStream;
use std::time::Duration;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {

let mut buffer = [0 as u8; 1024];
let mut stream = TcpStream::connect(&"127.0.0.1:8080".parse()?)?;

let poll = Poll::new()?;
let mut events = Events::with_capacity(128);

// Register the socket with `Poll`
poll.register(&stream, Token(0), Ready::writable() |     Ready::readable(),
              PollOpt::edge())?;

loop{
            poll.poll(&mut events, Some(Duration::from_millis(100)))?;
                for event in &events {
                    match event.token() {
                        Token(0) => {

                                 if event.readiness().is_readable()  {
                                     let read = stream.read(&mut buffer);
                                     match read {
                                         Ok(0) => {
                                             println!("server  socket  closed", );
                                             break
                                         },
                                         Ok(_n) => {
                                             println!("{}",std::str::from_utf8(&buffer)?);
                                         },
                                         Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock =>
                                             break,
                                         Err(_) => break
                                     }

                                  }

                                   if event.readiness().is_writable() {
                                         stream.write_all("i am client".as_bytes()).unwrap();

                                   }
                        },

                        _ => unreachable!()
                    } //match end.
                } //for end.

}//loop end.

}
