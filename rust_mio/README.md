# Rust MIO 0.6v 学习杂记

* 以MIO官方文档为主线

  `https://docs.rs/mio/0.6.21/mio/`

* MIO用法概要

  `MIO官方文档中给了一个代码例子说明基本用法`

```rust
use mio::*;
use mio::net::{TcpListener, TcpStream};

fn main() {
   
    // (1) Setup some tokens to allow us to identify which event is
    // for which socket.
    //这个token就是身份证号，唯一代表此soket.
    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);
    
    let addr = "127.0.0.1:13265".parse().unwrap();
    
    //(2) Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();
    
    //(3) Create a poll instance    
    //此poll离开作用域后，自动析构。
    let poll = Poll::new().unwrap();
    
    //(4) Start listening for incoming connections
    //注意：将token SERVER与sokcet server的唯一标定关系注册进Poller.
    //随后的事件处理loop中，每个event都持有一个token,用以标定自己属于那个soket.
    poll.register(&server, SERVER, Ready::readable(),
                  PollOpt::edge()).unwrap();
    
    // Setup the client socket
    let sock = TcpStream::connect(&addr).unwrap();
    
    // Register the socket
    poll.register(&sock, CLIENT, Ready::readable(),
                  PollOpt::edge()).unwrap();
    
    //(5) Create storage for events
    let mut events = Events::with_capacity(1024);
    
    loop {
        //(6) Wait for readiness events。
        //Blocks the current thread and waits for readiness events for any of the Evented handles that have been   //registered with this Poll instance. The function will block until either at least one readiness event has been received //or timeout has elapsed. A timeout of None means that poll will block until a readiness event has been received.
        poll.poll(&mut events, None).unwrap();
    
        for event in events.iter() {//(7) for each to handle event.
            match event.token() {
                SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let _ = server.accept();
                    println!("server accepted");
                }
                CLIENT => {
                    // The server just shuts down the socket, let's just exit
                    // from our event loop.
                    println!("client done");
                    return;
                }
                _ => unreachable!(),
            }
        }
    }
}
//for cargo.toml
//[dependencies]
//mio = "0.6.21"
```

> 
>
> Polls for readiness events on all registered values. 这句话意思明确，MIO是`就绪型事件模型`，比如：可读、可写等，告诉你可以开始读或写数据了！只是告诉你开始的时机，但苦力要自己做，所以称为Reactor事件模型；而Windows IOCP 属于'完成型事件模型'， 比如：数据读完了或写完了， 帮我们把苦力做完了，然后通知我们任务完成了，最后我们享用最终成果，此事件模型成为Proactor事件模型。
>
> MIO 非阻塞API, Non-blocking TCP, UDP。
>
> ``Token` is a wrapper around `usize` and is used as an argument to [`Poll::register`](https://docs.rs/mio/0.6.21/mio/struct.Poll.html#method.register) and [`Poll::reregister`](https://docs.rs/mio/0.6.21/mio/struct.Poll.html#method.reregister).`
>
> `token` cannot be `Token(usize::MAX)` as it is reserved for internal usage.
>
> 
>
> 上面代码例子属于单线程非阻塞Reactor事件处理模式。
>
> 同步异步之我的理解
>
> 对于同步，宏观说，做一件事分N步， 从1至N按部就班必须依次完成，如果在第i步卡住了，我们就只能等着，不能再向下一步推进，等待的方式可以分为`阻塞和非阻塞`，阻塞等待表示原地死等，其他什么也不干了；非阻塞等待表示我会一直等你，不再向下一步推进，但是等待期间我可以喝喝茶、看看电影、处理点别的事，只是我会时不时回来查问一下第i步的进展情况，若是搞定了，我就接着推进到i+1步， 若无进展，我接着做其他的事情。
>
> 微观将，比如soket data 到达了，OS通知你去内核领取，自己拿回家，那么此方式可以理解为同步事件模型，若是OS直接把数据送货上门了，敲门提醒你享用，此方式可以理解为异步事件模型。
>
> 对于异步，宏观说，做一件事分N步，同时推进N步，不分先后，哪一步完成了，通知我一下，然后我再决定后续处理流程。从微观讲，你发出一个系统调用，也不阻塞等待，直接返回！至于系统调用结果，未来通过回调函数另行通知你！
>
> 
>
> 详解：`poll.register(&stream, Token(0), Ready::readable() | Ready::writable(), PollOpt::edge())?;`
>
> * &stream, 代表一个soket.
>
> * Token(0) , 赋予此soket的唯一身份证号.
>
> *  Ready::readable() | Ready::writable(), 我们监听可读可写事件。
>
> * PollOpt::edge()) ； PollOpt::level()； PollOpt::oneshot(); 代表事件的派发方式，需要详细分析：
>
> 比如你poll后发现soket data 可读了，大小2kb, 你开始读取，只读取了1kg，不再读了， 也就是说socket buffer中还剩1kg data ,  此时你再次poll, 若是 PollOpt::level()， 则还是通知你soket data可读； 若是PollOpt::edge()) ，则不再通知你有数据可读了。
>
> 学过数字电子的可能了解，画过波形图，水平沿，上升沿；水平沿代表事件持续状态， 上升沿代表事件的转变；PollOpt::level()好比水平沿，只要事件没有转变，就会持续通知你此事件的存在！PollOpt::edge()好比上升沿，新旧事件交替，只通知你一次新事件出现，所以必须一次性把data读尽了，即直到read的时候返回WouldBlock， 表示数据读尽了，再读就要阻塞了。
>
> 
>
> PollOpt::oneshot() tells `Poll` to disable events for the socket after returning an event. so call poll again then to block . 要想重新监听此socket上的事件， 需要the socket would need to be reregistered using [`reregister`](https://docs.rs/mio/0.6.21/mio/struct.Poll.html#method.reregister).
>
> ```rust
> use mio::{Poll, Ready, PollOpt, Token};
> use mio::net::TcpStream;
> 
> let poll = Poll::new()?;
> let socket = TcpStream::connect(&"216.58.193.100:80".parse()?)?;
> 
> // Register the socket with `poll`, requesting readable
> poll.register(&socket, Token(0), Ready::readable(), PollOpt::edge())?;
> 
> // Reregister the socket specifying a different token and write interest
> // instead. `PollOpt::edge()` must be specified even though that value
> // is not being changed.
> poll.reregister(&socket, Token(2), Ready::writable(), PollOpt::edge())?;
> ```
>
> 
>
> ---
>
> pub fn [poll](https://docs.rs/mio/0.6.21/mio/struct.Poll.html#method.poll)(
>  &self,
>  events: &mut [Events](https://docs.rs/mio/0.6.21/mio/struct.Events.html),
>  timeout: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html)>
> ) -> [Result](https://doc.rust-lang.org/nightly/std/io/error/type.Result.html)<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>
>
> Wait for readiness events
>
> Blocks the current thread and waits for readiness events for any of the `Evented` handles that have been registered with this `Poll` instance. The function will block until either at least one readiness event has been received or `timeout` has elapsed. A `timeout` of `None` means that `poll` will block until a readiness event has been received.
>
> ```rust
> use mio::{Poll, Events};
> use std::time::Duration;
> 
> let poll = match Poll::new() {
>    Ok(poll) => poll,
>    Err(e) => panic!("failed to create Poll instance; err={:?}", e),
> };
> 
> // Create a structure to receive polled events
> let mut events = Events::with_capacity(1024);
> 
> // Wait for events, but none will be received because no `Evented`
> // handles have been registered with this `Poll` instance.
> let n = poll.poll(&mut events, Some(Duration::from_millis(500)))?;
> assert_eq!(n, 0);
> ```
>
> ---
>
> * Token的生成和用法例子
>
> Tokent is usize, which is a unsigned integer type,大小由OS Target决定，32bitOS usize is 4 byte, 64bitOS is 8byte. 
>
> ```rust
> use mio::{Events, Ready, Poll, PollOpt, Token};
> use mio::net::TcpListener;
> 
> use std::thread;
> use std::io::{self, Read};
> use std::collections::HashMap;
> 
> // After this number of sockets is accepted, the server will shutdown.
> const MAX_SOCKETS: usize = 32;
> 
> // Pick a token that will not be used by any other socket and use that one
> // for the listener.
> const LISTENER: Token = Token(1024);
> 
> // Used to store the sockets.
> let mut sockets = HashMap::new();
> 
> // This is used to generate a unique token for a socket
> let mut next_socket_index = 0;
> 
> // The `Poll` instance
> let poll = Poll::new()?;
> 
> // Tcp listener
> let listener = TcpListener::bind(&"127.0.0.1:0".parse()?)?;
> 
> // Register the listener
> poll.register(&listener,
>            LISTENER,
>            Ready::readable(),
>            PollOpt::edge())?;
> 
> // Spawn a thread that will connect a bunch of sockets then close them
> let addr = listener.local_addr()?;
> thread::spawn(move || {
>  use std::net::TcpStream;
> 
>  // +1 here is to connect an extra socket to signal the socket to close
>  for _ in 0..(MAX_SOCKETS+1) {
>      // Connect then drop the socket
>      let _ = TcpStream::connect(&addr).unwrap();
>  }
> });
> 
> // Event storage
> let mut events = Events::with_capacity(1024);
> 
> // Read buffer, this will never actually get filled
> let mut buf = [0; 256];
> 
> // The main event loop
> loop {
>  // Wait for events
>  poll.poll(&mut events, None)?;
> 
>  for event in &events {
>      match event.token() {
>          LISTENER => {
>              // Perform operations in a loop until `WouldBlock` is
>              // encountered.
>              loop {
>                  match listener.accept() {
>                      Ok((socket, _)) => {
>                          // Shutdown the server
>                          if next_socket_index == MAX_SOCKETS {
>                              return Ok(());
>                          }
> 
>                          // Get the token for the socket
>                          let token = Token(next_socket_index);
>                          next_socket_index += 1;
> 
>                          // Register the new socket w/ poll
>                          poll.register(&socket,
>                                       token,
>                                       Ready::readable(),
>                                       PollOpt::edge())?;
> 
>                          // Store the socket
>                          sockets.insert(token, socket);
>                      }
>                      Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
>                          // Socket is not ready anymore, stop accepting
>                          //必须单独处理这个错误类型，其代表socket还没有准备好，可能在未来某个时间准备好，
>                          //所以我们不必惊慌失措，就当没发生，接着loop就好。
>                          break;
>                      }
>                      //遇到错误就panic，实在不可接受，需要加入逻辑分析不同错误种类，只有不可恢复的error才需要panic.
>                      e => panic!("err={:?}", e), // Unexpected error
>                  }
>              }
>          }
>          token => {
>              // Always operate in a loop
>              loop {
>                  match sockets.get_mut(&token).unwrap().read(&mut buf) {
>                      Ok(0) => {
>                          // Socket is closed, remove it from the map
>                          sockets.remove(&token);
>                          break;
>                      }
>                      // Data is not actually sent in this example
>                      Ok(_) => unreachable!(),
>                      Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
>                          // Socket is not ready anymore, stop reading
>                          //不断 read data in loop直至data被读尽了，然后再次read data是就会出发这个WouldBlock错误种类，
>                          //意思说：没有data可读了。
>                          break;
>                      }
>                      e => panic!("err={:?}", e), // Unexpected error
>                  }
>              }
>          }
>      }
>  }
> }
> /*If operation fails with WouldBlock, then the caller should not treat this as an error, but instead should wait until another readiness event is received.*/
> ```
>
> ```bash
> cat /proc/sys/fs/file-max 
> more /proc/sys/fs/file-nr
> ```

**file-max** is the maximum File Descriptors (FD). It is a kernel setting enforced at the system level. **ulimit** is enforced at the user level. It should be configured to be less than file-max.

---

* 如何取消监听

```rust
use mio::{Events, Poll, Ready, PollOpt, Token};
use mio::net::TcpStream;
use std::time::Duration;

let poll = Poll::new()?;
let socket = TcpStream::connect(&"216.58.193.100:80".parse()?)?;

// Register the socket with `poll`
poll.register(&socket, Token(0), Ready::readable(), PollOpt::edge())?;

poll.deregister(&socket)?;

let mut events = Events::with_capacity(1024);

// Set a timeout because this poll should never receive any events.
let n = poll.poll(&mut events, Some(Duration::from_secs(1)))?;
assert_eq!(0, n);
```

---

* 如何监听自定义事件，即非系统事件，业务层面的事件

```rust
use mio::{Events, Ready, Registration, Poll, PollOpt, Token};
use std::thread;

//自定义一个事件，返回两个handle,一个用于poll注册监听此事件，另一个用于触发事件。
let (registration, set_readiness) = Registration::new2();

thread::spawn(move || {
    use std::time::Duration;
    thread::sleep(Duration::from_millis(500));

    set_readiness.set_readiness(Ready::readable()); //触发自定义事件
});

let poll = Poll::new()?;
//注册此自定义事件handle.
poll.register(&registration, Token(0), Ready::readable() | Ready::writable(), PollOpt::edge())?;

let mut events = Events::with_capacity(256);

loop {
    //监听自定义事件是否触发。
    poll.poll(&mut events, None);

    for event in &events {
        if event.token() == Token(0) && event.readiness().is_readable() {
            return Ok(());
        }
    }
}
```

`更全面的例子`

```rust
use mio::{Ready, Registration, Poll, PollOpt, Token};
use mio::event::Evented;

use std::io;
use std::time::Instant;
use std::thread;

pub struct Deadline {
    when: Instant,
    registration: Registration,
}

impl Deadline {
    pub fn new(when: Instant) -> Deadline {
        let (registration, set_readiness) = Registration::new2();

        thread::spawn(move || {
            let now = Instant::now();

            if now < when {
                thread::sleep(when - now);
            }

            set_readiness.set_readiness(Ready::readable());
        });

        Deadline {
            when: when,
            registration: registration,
        }
    }

    pub fn is_elapsed(&self) -> bool {
        Instant::now() >= self.when
    }
}

impl Evented for Deadline {
    //注册自定义事件
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        poll.register(&self.registration, token, interest, opts)
    }
  //重注册自定义事件
    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        poll.reregister(&self.registration, token, interest, opts)
    }
   //注销自定义事件
    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        poll.deregister(&self.registration)
    }
}
```

* set_readiness 错误例子，不该对事件派发顺序做任何假设和依赖，更不可以作为同步机制。

  所以事件派发出去了， 不要假设什么时候一定到！

> There is no guarantee that `readiness` establishes any sort of memory ordering. Any concurrent data access must be synchronized using another strategy.
>
> There is also no guarantee as to when the readiness event will be delivered to poll. A best attempt will be made to make the delivery in a "timely" fashion. For example, the following is **not** guaranteed to work:

```rust
use mio::{Events, Registration, Ready, Poll, PollOpt, Token};

let poll = Poll::new()?;
let (registration, set_readiness) = Registration::new2();

poll.register(&registration,
              Token(0),
              Ready::readable(),
              PollOpt::edge())?;

// Set the readiness, then immediately poll to try to get the readiness
// event
set_readiness.set_readiness(Ready::readable())?;

let mut events = Events::with_capacity(1024);
poll.poll(&mut events, None)?;

// There is NO guarantee that the following will work. It is possible
// that the readiness event will be delivered at a later time.
let event = events.get(0).unwrap();
assert_eq!(event.token(), Token(0));
assert!(event.readiness().is_readable());
```

* set_readiness 错误例子

```rust
use mio::{Registration, Ready};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (_registration, set_readiness) = Registration::new2();

    assert!(set_readiness.readiness().is_empty());

    set_readiness.set_readiness(Ready::readable())?; //不要假设事件立刻就被派发了!
    assert!(set_readiness.readiness().is_readable());// 事件可能没有派发到，也可能已经到了，总之不能依赖这种错误的	 //顺序假设。
    Ok(())
}

```

总之自定义事件和系统事件都需要implement Trait mio::event::Evented; 这样才可以注册到poll,实现监听；比如: mio::net::TcpStream

---

* # mio::net

  | [TcpListener](https://docs.rs/mio/0.6.21/mio/net/struct.TcpListener.html) | A structure representing a socket server                     |
  | ------------------------------------------------------------ | ------------------------------------------------------------ |
  | [TcpStream](https://docs.rs/mio/0.6.21/mio/net/struct.TcpStream.html) | A non-blocking TCP stream between a local socket and a remote socket. |
  | [UdpSocket](https://docs.rs/mio/0.6.21/mio/net/struct.UdpSocket.html) | A User Datagram Protocol socket.                             |

  非常简洁，用于提供跨平台统一一致的非阻塞socket API. 使用时遇到问题，请参阅`https://docs.rs/mio/0.6.21/mio/struct.Poll.html#portability`

1. socket监听

```rust
use mio::{Events, Ready, Poll, PollOpt, Token};
use mio::net::TcpListener;
use std::time::Duration;

let listener = TcpListener::bind(&"127.0.0.1:34255".parse()?)?;

let poll = Poll::new()?;
let mut events = Events::with_capacity(128);

// Register the socket with `Poll`
poll.register(&listener, Token(0), Ready::readable(),
              PollOpt::edge())?;

poll.poll(&mut events, Some(Duration::from_millis(100)))?;

// There may be a socket ready to be accepted
```

2. tcp流

```rust
use mio::{Events, Ready, Poll, PollOpt, Token};
use mio::net::TcpStream;
use std::time::Duration;

let stream = TcpStream::connect(&"127.0.0.1:34254".parse()?)?;

let poll = Poll::new()?;
let mut events = Events::with_capacity(128);

// Register the socket with `Poll`
poll.register(&stream, Token(0), Ready::writable(),
              PollOpt::edge())?;

poll.poll(&mut events, Some(Duration::from_millis(100)))?;

// The socket might be ready at this point
```

3. udp短报文

```rust
// An Echo program:
// SENDER -> sends a message.
// ECHOER -> listens and prints the message received.

use mio::net::UdpSocket;
use mio::{Events, Ready, Poll, PollOpt, Token};
use std::time::Duration;

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
    poll.poll(&mut events, Some(Duration::from_millis(100)))?; //阻塞当前线程，直至事件到达，或timeout.
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
}
```

> 上面这个例子， 编译，run, 你会发现terminal一直在滚动， 因为程序一直在send/echo,不断在循环！因为sender的socket已经初始化链接成功后，只要链路正常，就一直是可写的！所以socket可写事件不断生成和派发出来，故此echoer会不断收到可读事件。
>
> 我们也可以不去监听sender的可写事件， 而是在自己的代码逻辑中决定何时何地send message, 只不过socket是否可写，需要自己检测判断而已！通常也不需要事前检测， send失败了，可事后检查。因为mio poll判断可写了， 一定是条件满足， 可就在你开始send的时候链路断了，发送失败，也是非常可能的！所以发送状态需要自己及时检查。
>
> 对你无法完全掌控的事物，不要盲从自信，要做好事前和事后检测，从容应对失败，避免状态和逻辑混乱。



`都支持ipv6, ssl`

虽然在调用poll()之前做了一些socket初始化、监听、链接等启动操作， 但是直到调用poll()之时才是正真的启动运转，因为mio以已经做好了派发事件的准备！一切可以开始运转起来了！一定要深刻理解这一点，因为mio的一切都是围绕event loop运转的！

`另一个mio好文章: https://sergey-melnychuk.github.io/2019/08/01/rust-mio-tcp-server/`

`https://github.com/sergey-melnychuk/mio-tcp-server`

```rust
// Benchmarks:
// $ ab -n 1000000 -c 128 -k http://127.0.0.1:8080/
// $ wrk -d 30s -t 4 -c 128 http://127.0.0.1:8080/
//单线程eventloop.

use mio::net::{TcpListener, TcpStream};
use mio::{Poll, Token, Ready, PollOpt, Events};
use std::collections::HashMap;
use std::io::{Read, Write};

static RESPONSE: &str = "HTTP/1.1 200 OK
Content-Type: text/html
Connection: keep-alive
Content-Length: 6

hello
";

fn is_double_crnl(window: &[u8]) -> bool {
    window.len() >= 4 &&
        (window[0] == '\r' as u8) &&
        (window[1] == '\n' as u8) &&
        (window[2] == '\r' as u8) &&
        (window[3] == '\n' as u8)
}

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
    let mut buffer = [0 as u8; 1024];

    let mut events = Events::with_capacity(1024);
    loop {
        //poll阻塞当前线程。
        poll.poll(&mut events, None).unwrap();
        for event in &events {
            match event.token() {
                Token(0) => {
                    loop {
                        match listener.accept() {
                            Ok((socket, _)) => {
                                counter += 1;
                                let token = Token(counter);

                                //将accept出来的socket注册监听其可读事件。
                                //每个socket赋予一个身份证号token.
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
                                /*Read is performed in the loop until known WouldBlock error is returned. Each call to read returns (if successful) actual number of bytes read, and when there are zero bytes read - this means client has disconnected already, and there is no point if keeping the socket around (nor continuing the reading loop).*/
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
					
                    //start 从start到end这块代码逻辑，用于决定是否注册监听此socket的可写事件。
                    //说白了，业务逻辑按双方约定的protocol交谈， 此处server判断是否向client回话。
                    let ready = requests.get(&token).unwrap()
                        .windows(4)
                        .find(|window| is_double_crnl(*window))
                        .is_some();

                    if ready {
                        let socket = sockets.get(&token).unwrap();
                        //注意此socket在accept的时候已经被注册到poll, 所以此处需再注册。
                        //注意之前通过poll.register注册的信息会被poll.reregister再注册的信息取代。
                        //说白了，以poll.reregister为准。
                        poll.reregister(
                            socket,
                            token,
                            Ready::writable(),
                            PollOpt::edge() | PollOpt::oneshot()).unwrap();
                        //注意此处PollOpt::oneshot() 表示，就派发一次可写事件，
                        //事件派发后，注册的信息还保留在mio::poll中， 处于disable状态。
                    }
                    //end
                },
                token if event.readiness().is_writable() => {
                    requests.get_mut(&token).unwrap().clear();
                    sockets.get_mut(&token).unwrap().write_all(RESPONSE.as_bytes()).unwrap();

                    //终于此socket的可写事件到来了， 向客户端写完response后， 重新再注册监听此socket的可读事件。
                    //为什么需要重新注册呢？ 因为上面poll.reregister时指定了PollOpt::oneshot()参数， 所以可写事件到来之后，
                    //之前的注册的就被disable了， 不再派发事件，除非重新通过reregister方式enable 事件派发。
                    // Re-use existing connection ("keep-alive") - switch back to reading
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
/*笔者非要这么麻烦，每此以PollOpt::oneshot()方式注册监听client socket的可写事件吗？ 以edge或level方式一次性注册监听此client socket 的可写可读事件多好！也是呢！我的理解笔者可能想根据client发来的信息，是否满足条件， 然后再决定是否写response给client ；如果条件不满足，干脆也不会向poll注册，节省一些性能；如果不管这么多，可读可写都注册监听上， 然后在可写事件到来时，再判断是否满足条件写response给client，若不满足， 则poll做了无用功，浪费性能。

听起来好麻烦， 好绕口！因为你置身在mio eventloop中， 交由eventloop去驱动你的逻辑执行。你也可以采用其他方式，
比如，我只让poll监听client socket的可读事件， 你读取了client message之后，交给单独的线程、线程池、闭包、协成、future之类的独立执行单元去处理。还有一个问题，所有的client socket都交给主线程中的poll监听，当socket量大时，性能是否会下降！是否可以再开几个线程分别运行各自的poll, 然后把client socket分给它们分别监听处理？通过channel彼此互通。可行吗？ 需要再深入研究一下mio。

*/

```

> mio实现的是一个单线程事件循环，并没有实现线程池及多线程事件循环，如果需要线程池及多线程事件循环等需要自己实现。



* std::io::ErrorKind

> 上面例子中的read /write操作都属于io操作， 都返回Result<usize, Error>, Error is a struct in io module.
>
> 所以一旦出错，可由针对性处理，而不是简单的panic. 再次强调程序未按预期执行，请好好读一读：`https://docs.rs/mio/0.6.21/mio/struct.Poll.html#portability`

```rust
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    Other,
    UnexpectedEof,
}
```

* 如何准确判定peer socket已经关闭？

上面官方的例子中，只要检测到read的OK(0) ,就认为对端socket关闭了。

linux scoket 、 epoll、 mio等都是以tcp/ip为基础的！tcp/ip不是询问型协议，所以不能及时感知对端失效了，即使引用SO_KEEPALIVE SOCKET参数，也是2小时后才发送探测包，也就是说不能及时探知！tcp/ip之所以这样设计就是考虑到性能和带宽的问题！这可不是量子相干性，两个量子不管相隔多远，一个改变，另一个立即相应改变！我不是物理学家，粗浅理解如此！但现实中的问题是：网络中两个端点确定对方安好的方式唯有`询问`，不断询问，询问的频率越高越及时准确！但是代价高昂，就是严重浪费带宽！有得有失，看你的目标吧！所以说tcp/ip是可靠的， 但不是绝对可靠的！

所以一般这样做，(1)对于read, write之类操作结果要分析判断. (2) 加入timeout机制，特别是read/write等阻塞型api. (3) ignore signal SIGPIPE,check EPIPE result.  (4) 若调用了select/epoll之类，注意分析判断其给出的error   如: EPOLLRDHUP、EPOLLERR、EPOLLHUP (5) 加入心跳机制. 

---

我的理解，对于采用epoll之类的io复用，必须配合上非阻塞api, read/write等， 很显然呀，你不能读或写完某一个socket event后就block在那里了， 其他socket event怎么处理呀！当然单开个线程或采用异步IO也可以，总之不能block。

---

mio 4个需要特别注意点：

(1) With edge-triggered events, operations **must** be performed on the `Evented` type until [`WouldBlock`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.WouldBlock) is returned. edge模式时， 数据必须读尽，即不断read直到返回WouldBlock。

(2) Since even with edge-triggered events, multiple events can be generated upon receipt of multiple chunks of data, the caller has the option to set the [`oneshot`](https://docs.rs/mio/0.6.10/mio/struct.PollOpt.html#method.oneshot) flag.  将oneshot 与edge或level一起使用，可以避免饥饿问题；disable event 派发过来，不是丢弃，只是积压，处理完当前event后， 可以reregister此socket，从而允许后续事件派发。

(3) [`Poll::poll`](https://docs.rs/mio/0.6.10/mio/struct.Poll.html#method.poll) may return readiness events even if the associated [`Evented`](https://docs.rs/mio/0.6.10/mio/event/trait.Evented.html) handle is not actually ready. 

​	If operation fails with [`WouldBlock`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.WouldBlock), then the caller should not treat this as an error and wait until another 	readiness event is received.  //意思： poll可能会谎报军情，所以你的IO操作返回WouldBlock时，不是错误，忽略就好。

(4) The only readiness operations that are guaranteed to be present on all supported platforms are [`readable`](https://docs.rs/mio/0.6.10/mio/struct.Ready.html#method.readable) and [`writable`](https://docs.rs/mio/0.6.10/mio/struct.Ready.html#method.writable). 考虑到跨平台问题， mio只保证可读可写两事件在所有平台都支持。

---

level : write event 不断产生；edge: read event 不断产生；两者都会产生饥饿问题， 可以配合上oneshot参数和reregister，可有效避免产生饥饿问题。也可通过其他方式解决饥饿问题，如： 可以不现场处理，而只是将其打入队列中， 由独立的执行单元consume。 或者以闭包方式处理， 并将此闭包交给threadpool之类独立执行。还可以采用future方式， 交由独立executor执行。

---

我的理解：oneshot 模式触发后会disable 此socket事件的poll, 我实际测试过， 写个client不断向server send data,  当oneshot触发后， thread::sleep几分钟， 然后再reregister,  此期间client仍然不断send data, 但是过一会就block在那， 不再send data to server了， 直到server wake up 后执行reregister， 此刻server poll会派发一个此peer socket的可读事件， 然后server side 读取client data后， 再次sleep,  而client又恢复了发送数据，一会儿又block了， 如此反复。 我猜测client不断向server发送数据直到server端此peer socket data buffer满了，所以tcp协调停止client的发送，但是双方的链接是保持的。【精力有限，测试不太严谨，希望抛砖引玉】

`测试代码在：/rust_learn/rust_mio/mio_oneshot_test/`

---

同步阻塞： 等待数据到达， 数据到达后将其从kernel space copy to user space。

同步非阻塞： 查询数据是否到达，若否则立刻返回， 若是则将数据从kernel space copy to user space。

异步：客户说：我要数据，系统说：您别担心，先忙点别的，数据到了我送到你家里。

POSIX(可移植操作系统接口)把同步IO操作定义为导致进程阻塞直到IO完成的操作，反之则是异步IO

---

mio::poll 帮你等待数据， 而你需要copy data from kernel space to user space.

---

IO分两阶段：

```
1.数据等待阶段
2.内核空间复制回用户进程缓冲区阶段
```

一般来讲：阻塞IO模型、非阻塞IO模型、IO复用模型(select/poll/epoll)、信号驱动IO模型都属于同步IO，因为阶段2是阻塞的(尽管时间很短)。只有异步IO模型是符合POSIX异步IO操作含义的，不管在阶段1还是阶段2都可以干别的事。

---

【Unix网络编程 卷1 中几张经典的图】

![](https://github.com/yujinliang/rust_learn/blob/master/rust_mio/1.png)

---

![](https://github.com/yujinliang/rust_learn/blob/master/rust_mio/2.png)

---

![](https://github.com/yujinliang/rust_learn/blob/master/rust_mio/3.png)

---

![](https://github.com/yujinliang/rust_learn/blob/master/rust_mio/4.png)

---

![](https://github.com/yujinliang/rust_learn/blob/master/rust_mio/5.png)

---



poll.deregister(&socket).unwrap(); 是真正取消监听注册。

---

【学习笔记，不严谨， 疏于考证，难免谬误，欢迎指正】



* About me

> 作者：心尘了

> email: [285779289@qq.com](mailto:285779289@qq.com)

> 微信：13718438106



* Reference

  `https://docs.rs/mio/0.6.21/mio/index.html`
  
  `https://github.com/sergey-melnychuk/mio-tcp-server`
  
  `https://sergey-melnychuk.github.io/2019/08/01/rust-mio-tcp-server/`
  
  `https://www.cnblogs.com/euphie/p/6376508.html`
  
  `UNIX网络编程 卷1, (美)W. Richard Stevens著作`