use mio::net::{TcpListener, TcpStream};
use mio::{Poll, Token, Ready, PollOpt, Events};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{thread, time};

static RESPONSE: &str = "HTTP/1.1 200 OK
Content-Type: text/html
Connection: keep-alive
Content-Length: 6

hello
";

fn main() {
    let address = "0.0.0.0:8080";
    let listener = TcpListener::bind(&address.parse().unwrap()).unwrap();

    let poll = Poll::new().unwrap();
    poll.register(
        &listener,
        Token(0),
        Ready::readable(),
        PollOpt::edge()).unwrap();

    let mut counter: usize = 0;
    let mut sockets: HashMap<Token, TcpStream> = HashMap::new();
    let mut requests: HashMap<Token, Vec<u8>> = HashMap::new();
    let mut buffer = [0 as u8; 1];


    let mut events = Events::with_capacity(1024);
    loop {

        poll.poll(&mut events, None).unwrap();
        for event in &events {
            match event.token() {
                Token(0) => {
                    loop {
                        match listener.accept() {
                            Ok((socket, _)) => {
                                counter += 1;
                                let token = Token(counter);
                                //test.
                                println!("{:?}", socket);

                                poll.register(
                                    &socket,
                                    token,
                                Ready::readable(),
                                PollOpt::edge()).unwrap();

                                sockets.insert(token, socket);
                                requests.insert(token, Vec::with_capacity(192));
                            },
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock =>
                                break,
                            Err(_) => break
                        }
                    }
                },
                token if event.readiness().is_readable() => {
                    loop {
                        let read = sockets.get_mut(&token).unwrap().read(&mut buffer);
                        match read {
                            Ok(0) => {
                                sockets.remove(&token);
                                break
                            },
                            Ok(n) => {
                                let req = requests.get_mut(&token).unwrap();
                                for b in &buffer[0..n] {
                                    req.push(*b);
                                }
                            },
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock =>
                                break,
                            Err(_) => break
                        }
                    }
                        //for test.
                        let req = requests.get_mut(&token).unwrap();
                        println!("{:?}", req);
                        //------
                        let socket = sockets.get(&token).unwrap();
                        poll.reregister(
                            socket,
                            token,
                            Ready::writable(),
                            PollOpt::edge() | PollOpt::oneshot()).unwrap();

                },
                token if event.readiness().is_writable() => {
                    requests.get_mut(&token).unwrap().clear();
                    sockets.get_mut(&token).unwrap().write_all(RESPONSE.as_bytes()).unwrap();

                    let ten_millis = time::Duration::from_millis(100000);
                    thread::sleep(ten_millis);

                    poll.reregister(
                        sockets.get(&token).unwrap(),
                        token,
                        Ready::readable(),
                        PollOpt::edge()).unwrap();

                },
                _ => unreachable!()
            }
        }
    }
}
