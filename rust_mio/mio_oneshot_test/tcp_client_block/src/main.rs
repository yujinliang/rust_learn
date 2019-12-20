use std::net::TcpStream;
use std::io::{ Write};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

loop {
        println!("prepare send" );
        stream.write(&[1])?;
        println!("nex send" );
    }

} // the stream is closed here
