use mio::{Events, Ready, Poll, PollOpt, Token};
use mio::net::TcpStream;
use std::net::Shutdown;
use std::time::Duration;
use std::io::{Read, Write, ErrorKind};

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
                                     loop{ //coz edge trigger mode , then  do best to drain the data by  read loop.

                                     let read = stream.read(&mut buffer);
                                     match read {
                                         Ok(0) => {
                                             println!("read server  socket  closed", );
                                             poll.deregister(&stream).unwrap();
                                             stream.shutdown(Shutdown::Both).unwrap();
                                             return Ok(());
                                         },
                                         Ok(_n) => {
                                             println!("{}",std::str::from_utf8(&buffer)?);
                                         },
                                         Err(ref e) => {
                                              if e.kind() == std::io::ErrorKind::WouldBlock {
                                                  break;
                                              } else if e.kind() == ErrorKind::Interrupted {
                                                  continue;//restart!
                                               }
                                         }

                                     }

                                 } //end of read loop

                                }

                              if event.readiness().is_writable() {
                                  loop{
                                        match  stream.write_all("1".as_bytes()) {
                                            Err(e) => {
                                                    if e.kind() == ErrorKind::BrokenPipe {
                                                        println!("write server  socket  closed", );
                                                        poll.deregister(&stream).unwrap();
                                                        stream.shutdown(Shutdown::Both).unwrap();
                                                        return Ok(());
                                                    }
                                                    else if e.kind() == ErrorKind::WouldBlock {
                                                        break;
                                                    }
                                                    else if e.kind() == ErrorKind::WriteZero {
                                                        //should retry write operation until error occured!
                                                        continue;
                                                    }
                                            },
                                            _ => break,
                                        }
                                    } //end of write retry send loop!
                              }
                        },

                        _ => unreachable!()
                    } //match end.
                } //for end.

        }//loop end.

}
