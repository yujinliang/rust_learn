use std::net::{TcpStream, Shutdown};
use std::io::{ Write, Read, Result, ErrorKind, Error};

fn main() -> std::io::Result<()> {

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut buf = [0u8; 8];

loop {
        //stream.write(&[1])?;
        //hit BrokenPipe error , then close the peer socket.
        let ret = rio_writen(&mut stream, &[1,1,1,1,1,1,1]).unwrap();
        println!("write: {:?}", ret);

    //-----
        let rc = stream.read(&mut buf);
        println!("read: {:?}", rc);
        //check error to make decision to close the peer socket.
        match rc {
            Ok(n) => {
                println!("read, n: {}, server back {:?}" ,n, String::from_utf8_lossy(&buf));
                //read return with Ok(0) that means peer socket is dead in common case.
                //of course, we also  check  both Ok(0) and  Error::last_os_error().ErrorKind , such as: ErrorKind::NotConnected.
                //to avoid false alarm.
                if n == 0 {
                    stream.shutdown(Shutdown::Both).unwrap_or_default();
                    println!("peer socket closed {:?}", stream);
                    return Err(Error::last_os_error());
                }
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::ConnectionAborted |
                    ErrorKind::ConnectionRefused |
                    ErrorKind::ConnectionReset |
                    ErrorKind::NotConnected |
                    ErrorKind::BrokenPipe => {
                        //should close the socket
                        stream.shutdown(Shutdown::Both).unwrap_or_default();
                        println!("peer socket closed {:?}", stream);
                        return Err(e);
                    },
                    ErrorKind::TimedOut => {
                        //tow choice :  first , close the peer socket now.
                        //second send heart beat packet to peer  for checking its  alive.
                    },

                    //just for noneblocking mode.
                    //ErrorKind::WouldBlock => {continue;},

                    _ => { continue;}
                }
            }
        }

    }

} // the stream is closed here

fn rio_writen(stream :&mut TcpStream,  buf: &[u8]) -> Result<usize> {

        let mut nleft = buf.len();
        let mut  bufp = buf;
        while nleft > 0 {
                        //attention: nwritten hit Ok(0), that means maybe some wrong or nothing, but  always retry to write again and agian , until errors occureed(-1) or all Ok.
                    match  stream.write(bufp) {
                             Ok(n) => {
                                 nleft -= n ;
                                 if nleft > 0 {
                                     bufp = &bufp[n..];
                                }
                             }
                            e => {
                                return e;
                             }
                    }
        }
        return Ok(buf.len());
}
