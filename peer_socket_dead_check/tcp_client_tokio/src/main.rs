use tokio::net::{TcpStream};
use tokio::prelude::*;
use std::io::{ErrorKind};
use std::net::{Shutdown};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut buf = [1u8; 10];
    loop {
        // Write some data.
        match  stream.write_all(&buf).await {
            Err(e) => {
                println!("w: {:?}", e);
                if e.kind() == ErrorKind::ConnectionAborted {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                else if e.kind() == ErrorKind::ConnectionRefused {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                else if e.kind() == ErrorKind::ConnectionReset {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                else if e.kind() == ErrorKind::BrokenPipe {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                } 
                else {// if e.kind() == ErrorKind::Interrupted {
                            // if e.kind() == ErrorKind::WriteZero {
                    continue;
                }
            },
            _ => {},
        }
        
        //---
        match   stream.read(&mut buf).await {
            Err(e) => {
                println!("r: {:?}", e);
                if e.kind() == ErrorKind::ConnectionAborted {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                else if e.kind() == ErrorKind::ConnectionRefused {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                else if e.kind() == ErrorKind::ConnectionReset {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                } 
                else if e.kind() == ErrorKind::NotConnected {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                else {
                    continue;
                }
            
            }
            Ok(n) => {
                println!("r:{:?}", n);
                if n == 0 {
                    //peer socket is dead
                    stream.shutdown(Shutdown::Both)?;
                    break;
                } 
            }
        }
        println!("{:?}", String::from_utf8_lossy(&buf));
        
    }
    Ok(())
}
