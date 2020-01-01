# How to verify the peer socket is dead?

* 前言

> 对于`TCP/IP`网络编程而言，我们在读写数据时，当然希望获知对端是否还活着！量子纠缠态当然很是理想，但是现实网络世界中，两个不可见端点互相通讯，确定对方还活着的方法就是不断询问！我问-你答， 我问-你不答，我就当你死了， 所以对于`TCP/IP`网络编程而言， read最好有timeout机制保护，避免server无限制浪费资源去等待可能早已失效的对端；若业务层需要及时感知对端已死， 则需要`心跳包`机制， 典型的牺牲空间换取时间！对于`linux raw sokcet , epoll, (同步阻塞)std::net::tcpstream/（同步非阻塞）mio::net::tcpstream/（异步）tokio::net::tcpstream`而言， 对其接口调用返回结果和`errno`的判断是必须的！以此来检测对端是否还活着！
>
> 但是这都建立在两端`TCP/IP协议栈`可以正常交互的基础之上，比如4次关闭握手等！如果对端突然断点，网线脱落等， 则远端不会得到通知， 除非远端一直在向本端发送心跳包！
>
> `TCP/IP协议`不是轮询型协议， 它不会主动帮你探测对端是否还活着！它被设计的目标之一就是对网络不做太多假设， 比如一定可达之类！它只是按照路由协议尽可能地找路，如果没路可达（对端已死也是无路可达），它就通过`ICMP`向本端报告无路可达！
>
> 对于需要发送的数据包，它提供校验， 顺序， 确认/重发， 收发窗口等主要机制，尽可能保证传输的可靠性！所以它不是绝对可靠的！`再次强调TCP/IP不是轮询型协议`， 虽然你可以对socket 设定`SO_KEEPALIVE`选项，命令`TCP/IP`帮你定时向对端发送心跳包，从而尽早得知对端生死！但是这个频率默认是2小时左右发一次！后来据说可以设定这个时间间隔更小一些！但是我不太赞成让`TCP/IP`协议栈帮你做！最好在自己的业务层代码中实现！我查阅了一些资料，也做了一些小实验， 不全面， 所以不敢说一定严谨， 只是归总出来，以备日后我参考方便而已！对于read/write等操作返回的：`0 、-1、Ok(n)、 Err(e)`等不同取值情况的准确含义简单归纳一下。



* 一般性总结

> 不论`linux socket/epoll/select/poll`亦或Rust网络库， 对于read操作的返回值n， 0 代表对端关闭， 小于零代表出错， 具体什么错误需结合`linux errno`判断，比如中断则忽略！ 对于Rust的`Ok(0)`代表对端关闭， Err(e)代表出错，一般`crate libc` 才可能返回`Ok(-1)`代表底层出错，需要结合`std::io::Error::last_os_error().kind()`来决定错误具体应对， 通常只要检测`Err(e) && e.kind()`即可， 具体的错误代码都定义在`std::io::ErrorKind`中， 里面明确定义几个`Error Case`代表网络不通，一看便知，如：`ConnectionReset/ConnectionAborted/NotConnected/ConnectionRefused`等,  其中`Interrupted (EINTR)` 代表中断发生， 我们可以重启read操作！`TimedOut`代表超时，意味着我们是否需要主动发送心跳包！`WouldBlock`主要针对`非阻塞API`，用于提示我们数据已经读尽，若再调用read则可能block,特别是`linux epoll edge and mio edge `模式，需要不断循环read直到返回``WouldBlock``代表排干数据，从而避免数据丢失。
>
> ---
>
> 对于write操作的返回值， 0 代表出错（对端关闭），还是正常，许多资料观点对立不统一！但是各种write return value的描述， 基本描述一致，即write返回值n, 满足` n <= buf.len()` ，既然是可能返回小于缓存长度的值， 那么返回0，当然也就是正常的！但是0又意味着什么也没有发出去，这又不正常，所以`std::io::ErrorKind::WriteZero`对应这个情况， 当然如果`buf.len() == 0`, 那么write也会返回0，所以我这样处理， 只要满足`0 =< n < buf.len()`我就重试发送，直到发送成功或错误暴露出来， 如：`BrokenPipe/ConnectionRefused/ConnectionReset/ConnectionAborted/NotConnected`等代表对端已死！



* write/send误解

> 我们想当然地认为只要write/send成功，就认定data已达对端，这是错误的！write只是copy data from user space to kernel space! data被加入到系统的发送缓冲区等待发送而已！data可能在未来发送失败！所以如果我们的业务层需要保证data确实已被对端接受的话， 需要设计好业务层的确认协议！比如`Mysql client/server protocol`会给你的命令请求，回发`Ok/Err/Eof packet` , 这种业务层交互协议，就是应对write/send的不可靠！



* write/send必须处理`signal SIGPIPE`信号

> 一般屏蔽忽略此信号：`signal(SIGPIPE, SIG_IGN);` 随后再调用write可能返回`EPIPE errno`或者``ErrorKind::BrokenPipe` 代表对端关闭不通！其他屏蔽忽略此信号方法：`signal(SIGPIPE), setsocktopt(SO_NOSIGPIPE), and  send(MSG_NOSIGNAL),pthread_sigprocmask()`



* 有所不能

> 总结： 不论是`socket 亦或epoll`之类， 亦或rust的同步和异步IO, 亦或什么`eventloop, reactor/proactor`之类， 对于远端突然断电，网线松动都没有办法及时感知， 除非业务层自己发送心跳包和超时机制。



* `记住网络字节序就是大端字节序`

>  网络字节序就是大端字节序，这是标准规定。
>
>  主机字节序： 小端字节序或大端字节序， 一般`intel x86`的主机字节序是小端字节序。
>
>  字节序针对：数字（整数和浮点数）
>
>  大小端字节序定义， 及判断代码 in c/c++
>
>  Crate [byteorder](https://docs.rs/byteorder/1.3.2/byteorder/) for rust
>
>  
>
>  {小端口诀： 高对高，低对低。
>
>  大端口诀：高对地，低对高
>
>  内存地址都是从低向高增长}
>
>  
>
>  [verify byte order]
>
>  ```c++
>  const int i = 1; 
>  #define is_bigendian() ( (∗(char∗)&i) == 0 )
>  ```
>
>  ```c++
>  bool is_big_endian(void) {  
>      union {       
>          uint32_t i;    
>          char c[4];   
>      } bint = {0x01020304};   
>      return bint.c[0] == 1;  
>  }
>  ```
>
>  [byte order reverse]
>
>  ```c++
>  short reverseInt (char ∗c) { //c for big endian order for network
>      int i;
>      char ∗p = (char ∗)&i; //p for local computer
>  
>      if (is_bigendian()) {
>          p[0] = c[0];
>          p[1] = c[1];
>          p[2] = c[2];
>          p[3] = c[3];
>      } else {
>          p[0] = c[3];
>          p[1] = c[2];
>          p[2] = c[1];
>          p[3] = c[0];
>      }
>  ```
>
>  



> `Linux epoll EPOLLERR、EPOLLHUP、EPOLLRDHUP`   资料说代表对端关闭，我不太熟悉。



> `Linux epoll edge and Rust mio Poll edge trigger mode`  we need loop read until return -1 and `errno` with `EAGAIN `or ``WouldBlock`` , that means drain the data by best to avoid data lost! 而且可读事件可能会蜂拥而来，需要采用线程、闭包、协程、future之类来处理，避免延迟和饥饿。
>
> 对于level mode 可能造成可写事件蜂拥而至，需要借助`oneshot模式`来对付，避免无畏的浪费。



* `TCP/IP`不是轮询型协议

> 切记、切记，`TCP/IP`不会帮你一直盯着对端的生死，不要误解面向链接协议的意思！它的大概意思是说， 通讯之前先要确定两端都活着，都在， 而且两端之间有路可达而已，具体到可靠性也只是针对数据包而言，比如包数据的校验、包的次序、包的确认重发、包的收发窗口，包的拥塞控制等，所以它不负责维持一个链路，因为`TCP/IP`的设计目标之一就是不对底层传输网络做过多假设！避免网络链接被中断！它只是负责找到路就好，不管怎么绕，有路可达就好， 否则不通！所以`TCP/IP`的可靠性在于尽可能找到路，并给数据包编号，对端收到后按包序号再组装！如果丢失则会确认重传。
>
> 结论是：如果你需要及时获知对端生死， 请自己发送心跳包并等待对端反馈！主动发包意味着要`TCP/IP`为你的包包找路， 有路则对端就活着，没路（包括对端端口进程死了）的话，就是对端死了！当然`TCP/IP`协议本身也不能立刻就断定对端死了， 它也会重试多次之后才会认定！所以也是有一定延迟的！看来还是量子纠缠态比较理想呀！

---



* TIME_WAIT状态 和 `SO_REUSEADDR`选项`http://man7.org/linux/man-pages/man7/socket.7.html`

TCP主动关闭一方，将会进入TIME_WAIT，这个状态需要维持`2MSL`时间才会断开。`MSL`叫最大段生命周期，也就是一个TCP段在网络存活的最大时间。RFC 1122建议是2分钟，但BSD sockets的实现把`MSL`设定为30秒。`2MSL`也就是60秒。为什么主动关闭一方需要维持`2MSL`再关闭呢：

1. 实现可靠的中断：被动方(server)发送FIN后，主动方(client)回应`ACK`，如果这个`ACK`丢了，被动方还会重新发送FIN，TIME_WAIT状态就是预留足够的时间应对中断时的丢包重传。
2. 避免新的连接接受网络中旧的重复包：TCP由于重传机制，可能会产生重复包，且这些包可能会在连接断开后才到达；如果刚好有一个新的连接使用同样的`IP/port`，这些重复包就会传给新的连接，TCP必须避免这种情况发生，它的处理方法是如果TCP端处于TIME_WAIT状态，无法建立新的连接。`TCP/IP包的序号机制也可有效识别丢弃旧包！`

如果服务器bind一个地址，这个地址刚好有一个TCP处于TIME_WAIT的状态，bind会返回`EADDRINUSE`(Address already in use)错误，这种情况出现在，服务器与客户端的连接socket执行了主动关闭，这个socket的TCP会处于TIME_WAIT状态，然后服务器重启尝试去bind这个地址，系统默认不允许bind， 解决办法是对这个socket使用`SO_REUSEADDR`选项，` SO_REUSEADDR`允许同一个端口上绑定多个`IP`，只要这些`IP`不同。



* `Linux epoll EPOLLEXCLUSIVE （ since Linux 4.5）`

  用于缓解`epoll`惊群问题。详情参看`http://man7.org/linux/man-pages/man2/epoll_ctl.2.html`

  

* `SO_REUSEPORT(since Linux 3.9) ` 详情参看`http://man7.org/linux/man-pages/man7/socket.7.html`

  允许多个进程线程监听同一个`ip:port`, 而且内核提供了一定的负载均衡能力。

> ```c
>               Permits multiple AF_INET or AF_INET6 sockets to be bound to an
>               identical socket address.  This option must be set on each
>               socket (including the first socket) prior to calling bind(2)
>               on the socket.  To prevent port hijacking, all of the pro‐
>               cesses binding to the same address must have the same effec‐
>               tive UID.  This option can be employed with both TCP and UDP
>               sockets.
> 
>               For TCP sockets, this option allows accept(2) load distribu‐
>               tion in a multi-threaded server to be improved by using a dis‐
>               tinct listener socket for each thread.  This provides improved
>               load distribution as compared to traditional techniques such
>               using a single accept(2)ing thread that distributes connec‐
>               tions, or having multiple threads that compete to accept(2)
>               from the same socket.
> 
>               For UDP sockets, the use of this option can provide better
>               distribution of incoming datagrams to multiple processes (or
>               threads) as compared to the traditional technique of having
>               multiple processes compete to receive datagrams on the same
>               socket.
> ```



* 惊群

>从socket accept（`since linux 2.6`） 到` epoll wait`, 通俗地讲，只要多进程或多线程中， 大家都在关注同一个target, 如同一个`socket fd`or  `other fd`等,  大家都block在那监听， 一旦这个target有事件到来，大家都惊醒了， 确切地说是被操作系统唤醒的， 但是只有一个可以抢到这个target的处理权！其他没抢到的要重新睡去或是干点别的！这实在是浪费资源！操作系统无意义地调度大家醒来！此种情况被称为惊群效应。Linux系统都基本解决了！上面的参数选项就是。以前的解决方法就是加锁，大家醒来都去抢一把锁，只有一个可以抢到锁，谁抢到锁谁处理（accept）！不过加锁还是有开销的；Linux自己不做无用功岂不是更好。
>
>这篇文章讲的好：`https://zhuanlan.zhihu.com/p/88181936`
>
>1. `epoll_create called before fork`
>
>每个子进程共享同一个`epoll红黑树` ， 此类惊群Linux已解决， 因为任意一个进程都可以accept这个新连接， 并且内核可以将新连接与此进程绑定， 随后的读写事件也都由此进程处理，比较清晰明确。
>
>
>
>2. `epoll_create called after fork`
>
>每个子进程有自己独立的`epoll 红黑树`， 而且对于每一个`fd`, 内核不好做出准确清晰推断，比如：accept/定时/读写等等，所以只好唤醒每个子进程，内核说，你们自己看着办吧！`EPOLLEXCLUSIVE`这个新加入的选项，也只是缓解一下此类惊群问题！没有完全避免！只能每次尽可能少惊醒几个！从而减少了浪费。
>
>通常通过共享同一个`socket fd`来实现多个进程或线程共同监听同一个target，比如`Ip:port` , 大家一起来分流这个target的事件和数据！从而实现更高的处理效率，但是也引出了惊群问题！`SO_REUSEPORT`选项非常有用处，我们不用共享同一个`socket fd`, 而是每一个进程和线程`create/bind/listen/accept`自己的`socket fd`, 各自独立！大家都可以绑定监听同一个`ip:port`, 内核帮我们做负载均衡，大家不必醒来去抢了！内核会做出选择，然后唤醒某个！从而可以彻底避免惊群问题！详情请看`http://man7.org/linux/man-pages/man7/socket.7.html`
>
>以上的描述都在说进程， 实则也包含线程，同理。







【学习笔记，不严谨， 疏于考证，难免谬误，欢迎指正】

- About me

> 作者：心尘了

> email: [285779289@qq.com](mailto:285779289@qq.com)

> 微信：13718438106



* Reference

  `http://man7.org/linux/man-pages/man7/socket.7.html`

  `http://man7.org/linux/man-pages/man2/epoll_ctl.2.html`

  `https://www.ibm.com/support/knowledgecenter/en/SSLTBW_2.2.0/com.ibm.zos.v2r2.bpxbd00/rtwri.htm`

  `https://www.ibm.com/support/knowledgecenter/en/SSLTBW_2.3.0/com.ibm.zos.v2r3.bpxbd00/rtrea.htm`

  `https://developer.ibm.com/tutorials/l-sockpit/`

  `https://doc.rust-lang.org/std/io/enum.ErrorKind.html`

  `https://stackoverflow.com/questions/3081952/with-c-tcp-sockets-can-send-return-zero`

  `https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write`

  `http://man7.org/linux/man-pages/man2/write.2.html`

  `https://laptrinhx.com/tcp-ip-sockets-and-sigpipe-3327339302/`

  `https://www.cnblogs.com/jiu0821/p/7678132.html`

  `https://blog.csdn.net/qq_18998145/article/details/96479368`

  `https://zhuanlan.zhihu.com/p/71799852`

  `https://www.cnblogs.com/myd620/p/6219994.html`

  `https://blog.csdn.net/yongqingjiao/article/details/78819791`

  `https://www.cnblogs.com/embedded-linux/p/7468442.html`

  `https://zhuanlan.zhihu.com/p/62389040`
  
  `https://zhuanlan.zhihu.com/p/61652809`
  
  `https://blog.csdn.net/u012398362/article/details/102747350`
  
  `https://blog.didiyun.com/index.php/2018/11/27/linux-nginx/`
  
  `https://zhuanlan.zhihu.com/p/88181936`
  
  `https://zhuanlan.zhihu.com/p/53374516`