# Rust MIO 学习杂记

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
>   比如你poll后发现soket data 可读了，大小2kb, 你开始读取，只读取了1kg，不再读了， 也就是说socket buffer中还剩1kg data ,  此时你再次poll, 若是 PollOpt::level()， 则还是通知你soket data可读； 若是PollOpt::edge()) ，则不再通知你有数据可读了。
>
>   学过数字电子的可能了解，画过波形图，水平沿，上升沿；水平沿代表事件持续状态， 上升沿代表事件的转变；PollOpt::level()好比水平沿，只要事件没有转变，就会持续通知你此事件的存在！PollOpt::edge()好比上升沿，新旧事件交替，只通知你一次新事件出现，所以必须一次性把data读尽了，即直到read的时候返回WouldBlock， 表示数据读尽了，再读就要阻塞了。
>
>   
>
>   PollOpt::oneshot() tells `Poll` to disable events for the socket after returning an event. so call poll again then to block . 要想重新监听此socket上的事件， 需要the socket would need to be reregistered using [`reregister`](https://docs.rs/mio/0.6.21/mio/struct.Poll.html#method.reregister).
>
>   ```rust
>   use mio::{Poll, Ready, PollOpt, Token};
>   use mio::net::TcpStream;
>   
>   let poll = Poll::new()?;
>   let socket = TcpStream::connect(&"216.58.193.100:80".parse()?)?;
>   
>   // Register the socket with `poll`, requesting readable
>   poll.register(&socket, Token(0), Ready::readable(), PollOpt::edge())?;
>   
>   // Reregister the socket specifying a different token and write interest
>   // instead. `PollOpt::edge()` must be specified even though that value
>   // is not being changed.
>   poll.reregister(&socket, Token(2), Ready::writable(), PollOpt::edge())?;
>   ```
>
>   
>
>   ---
>
>   pub fn [poll](https://docs.rs/mio/0.6.21/mio/struct.Poll.html#method.poll)(
>     &self,
>     events: &mut [Events](https://docs.rs/mio/0.6.21/mio/struct.Events.html),
>     timeout: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html)>
>   ) -> [Result](https://doc.rust-lang.org/nightly/std/io/error/type.Result.html)<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)>
>
>   Wait for readiness events
>
>   Blocks the current thread and waits for readiness events for any of the `Evented` handles that have been registered with this `Poll` instance. The function will block until either at least one readiness event has been received or `timeout` has elapsed. A `timeout` of `None` means that `poll` will block until a readiness event has been received.
>
>   ```rust
>   use mio::{Poll, Events};
>   use std::time::Duration;
>   
>   let poll = match Poll::new() {
>       Ok(poll) => poll,
>       Err(e) => panic!("failed to create Poll instance; err={:?}", e),
>   };
>   
>   // Create a structure to receive polled events
>   let mut events = Events::with_capacity(1024);
>   
>   // Wait for events, but none will be received because no `Evented`
>   // handles have been registered with this `Poll` instance.
>   let n = poll.poll(&mut events, Some(Duration::from_millis(500)))?;
>   assert_eq!(n, 0);
>   ```
>
> ---
>
> * Token的生成和用法例子
>
>  Tokent is usize, which is a unsigned integer type,大小由OS Target决定，32bitOS usize is 4 byte, 64bitOS is 8byte. 
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
>               LISTENER,
>               Ready::readable(),
>               PollOpt::edge())?;
> 
> // Spawn a thread that will connect a bunch of sockets then close them
> let addr = listener.local_addr()?;
> thread::spawn(move || {
>     use std::net::TcpStream;
> 
>     // +1 here is to connect an extra socket to signal the socket to close
>     for _ in 0..(MAX_SOCKETS+1) {
>         // Connect then drop the socket
>         let _ = TcpStream::connect(&addr).unwrap();
>     }
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
>     // Wait for events
>     poll.poll(&mut events, None)?;
> 
>     for event in &events {
>         match event.token() {
>             LISTENER => {
>                 // Perform operations in a loop until `WouldBlock` is
>                 // encountered.
>                 loop {
>                     match listener.accept() {
>                         Ok((socket, _)) => {
>                             // Shutdown the server
>                             if next_socket_index == MAX_SOCKETS {
>                                 return Ok(());
>                             }
> 
>                             // Get the token for the socket
>                             let token = Token(next_socket_index);
>                             next_socket_index += 1;
> 
>                             // Register the new socket w/ poll
>                             poll.register(&socket,
>                                          token,
>                                          Ready::readable(),
>                                          PollOpt::edge())?;
> 
>                             // Store the socket
>                             sockets.insert(token, socket);
>                         }
>                         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
>                             // Socket is not ready anymore, stop accepting
>                             break;
>                         }
>                         e => panic!("err={:?}", e), // Unexpected error
>                     }
>                 }
>             }
>             token => {
>                 // Always operate in a loop
>                 loop {
>                     match sockets.get_mut(&token).unwrap().read(&mut buf) {
>                         Ok(0) => {
>                             // Socket is closed, remove it from the map
>                             sockets.remove(&token);
>                             break;
>                         }
>                         // Data is not actually sent in this example
>                         Ok(_) => unreachable!(),
>                         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
>                             // Socket is not ready anymore, stop reading
>                             break;
>                         }
>                         e => panic!("err={:?}", e), // Unexpected error
>                     }
>                 }
>             }
>         }
>     }
> }
> ```
>
> ```bash
>  cat /proc/sys/fs/file-max 
>  more /proc/sys/fs/file-nr
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
        self.registration.register(poll, token, interest, opts)
    }
  //重注册自定义事件
    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt)
        -> io::Result<()>
    {
        self.registration.reregister(poll, token, interest, opts)
    }
   //注销自定义事件
    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        poll.deregister(&self.registration)
    }
}
```

