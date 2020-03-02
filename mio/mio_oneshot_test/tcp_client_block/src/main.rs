use std::net::TcpStream;
use std::io::{ Write, Read};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut buf = [0u8; 8];

loop {
        println!("prepare send" );
        stream.write(&[1])?;
    //-----
        stream.read(&mut buf)?;
        println!("server back {:?}" , String::from_utf8_lossy(&buf));
    }

} // the stream is closed here
