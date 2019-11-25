# Linux Signal 处理 in Rust

1. Linux Signal 特点：

   异步性、并发性、无序性。`POSIX.1`允许系统递送该信号一次或多次。Linux是这样实现的：常规信号在递达之前产生多次只计一次，而实时信号在递达之前产生多次可以依次放在一个队列里。在单线程时代，编写信号处理程序就是一件棘手的事情，更何况多线程时代，由于Signal会打断正在运行的线程，包括Signal handler也会被其后的信号打断！所以Signal展现了明显的异步和并发特性，所以要求Linux Signal Handler中只可以call '`异步信号安全函数`， 具体而言就是：`可重入函数`和`信号不可中断函数（即原子性）`， 所以Signal Handler中是个严格受限的执行环境；总之需要谨慎对待。

   

2. 在Linux Signal中不允许的call有哪些？

   * `Pthreads`函数族
   * 通过condition variable通知其他线程
   *  `malloc()/free()、exit()`
   * `printf()、sprintf()、scanf()、strsignal()`、各种log等标准IO库函数，但`write()`可用
   * `Mutex`等锁，但可用`volatile sig_atomit_t`原子类型。
   * 全局变量（`volatile sig_atomit_t`类型允许使用）
   * 静态变量
   * 一般主流的库和OS System Call都会标注是否满足async-signal-safe function, 如果无法确认，就不要用。
   
   > 原则来讲，signal handler中可调用，要么是`纯代码只引用局部栈资源和状态`，要么是`可重入函数`， 要么是`信号不可中断函数`，只此三者无其他。



> 注意：因为信号可在任何时候任何代码执行位置中断某thread执行流，所以只要不是`原子性（即信号不可中断）操作`都不可靠，故一般锁若非原子性的也不可靠；故须避免deadlock、race condition亦或全局共享数据状态被破坏。所以signal handler实现得越简单越好，赶快从`信号异常流`回归到`线程正常执行流`中， 总原则：得知信号发生后，回到正常执行流中再进行处理！手脚不再束缚。
>
> 如果拿不准，则可以分析将要调用的函数或库实现是否依赖全局共享或静态数据状态，则最好不要用在signal handler中。一个异步信号安全函数要么是可重入函数， 要么是原子的， 既不可以被信号中断执行的。



3. 可重入函数

> 重入即表示重复进入，首先它意味着这个函数可以被中断，其次意味着它除了使用自己栈上的局部变量以外不依赖于外部环境（包括static和全局变量），可以允许有多个该函数的副本在同时运行，由于它们使用的是分离的栈，所以不会互相干扰。
>
> A reentrant function:
>
> - Does not hold static data over successive calls
> - Does not return a pointer to static data; all data is provided by the caller of the function
> - Uses local data or ensures protection of global data by making a local copy of it
> - Must not call any non-reentrant functions
>
> 总结：说白了，可重入函数实现要求只允许使用`局部量`， 不允许使用`全局和静态量`， 而且只允许调用可重入函数；这是非常苛刻的，所以可重入函数非常少。故此： `可重入函数`一定就是`线程安全函数`和`异步信号安全函数`， 但是反之未必！因为`线程安全函数`可以使用`全局和静态量`，只是加锁保护了，强制多线程有序使用，不准争抢混乱，但是不满足`可重入`要求。`信号不可中断函数`只是强调本函数`信号不可中断`，亦非要求不准使用`全局和静态量`；`线程安全`强调对`全局资源或状态`的有序使用，并非`信号不可中断`，所以锁不是牢不可破的，除非它是`原子锁atomic*` , 所以说：`异步信号安全函数`未必就是`线程安全函数`， 反之亦然！

> 最后再啰嗦一下`信号与多线程`，他俩不对付！`信号`产生于单线程时代，等于说信号一来，则整个进程就被中断执行了！但是信号的异步性，并发性，随机性等特征导致那时编写信号处理程序就要十分谨慎棘手！
>
> 直到多线程出现，创造了Mutx、 ReadWrite Mutex、Spin Mutex 、Condition Variable等线程互斥同步工具，终于降服了多线程，令其和平有序相处！但是`异步信号`出来搅局了！它无序、随时在任何位置中断某一个线程的执行流！要命的是，上面列出的各种线程互斥同步工具都可能被信号中断，如果signal handler中与此线程引用了相同的互斥同步元素，那么就可能会出现`状态错乱破坏问题`， 如：某线程进入加锁操作， 但信号突然发生，加锁操作若不是原子的亦或信号不可中断的， 则必然被中断执行，此时内部数据状态不完整，但是此时signal handler开始执行，若是也引用了这个锁，则也开始加锁操作， 而且此signal handler也同样可以被后来的其他signal中断， 所以这个加锁操作很可能会错上加错，乱上加乱！当然上面描述的事情也可能从未出现， 偶尔程序死锁了也只是重启一下，并未多心信号导致！
>
> 当然经过前人的不断努力和操作系统的进步，许多问题和漏洞都可以补救或屏蔽！但问题仍很复杂！所以有人开玩笑说最好不用信号，或者简单忽略，或者阻塞， 亦或只是在signal handler中简单地`_exit()`,或只设置一个标志量！总之signal handler实现得越简单越好！我的荒谬之语，诸君可笑而了之！



4. Linux Signal提示
   * 因为signal不排队，无序，会丢弃，所以不可以用它做可靠计数，比如处理`SIGCHLD`
   
   * 最好不用signal做进线程间通信，不如监听socket或文件描述符
   
   * 不要使用基于signal实现的定时函数，如：alarm/ualarm/sleep/usleep/timer_create等
   
   * 不主动处理各种异常信号，只用默认语义，除非必须处理，如：网络程序必须处理SIGPIPE.
   
   * 最好不用signal, 或者保持signal handler足够简单，比如只是设定一个标志（`volatile sig_atomit_t`）
   
   * 但是对于server programming , 信号：SIGPIPE 必须处理。
   
     >  signal(SIGPIPE, SIG_IGN); //on linux.
     >
     > So as you can see, solving this issue is *hard*. My recommendation for your code is to use all three techniques: *signal(SIGPIPE)*, s*etsocktopt(SO_NOSIGPIPE)*, and *send(MSG_NOSIGNAL)*, surrounded by the appropriate *#ifdefs*. It's an annoying set of things you have to do, but it's a non-optional thing you need to handle correctly, that must survive later programmers who may not understand this issue.
     
   * SIGSTOP and SIGKILL cannot be caught or ignored or blocked!
   
   * ##### SIGCONT and SIGSTOP is to be used by Debugger for implementing breaks and continue.
   
   * After a fork() in a multithreaded program, the child can safely call only async-signal-safe functions (see signal-safety(7)) until such time as it calls execve(2).
   
   



5. Interruption and Restarting of System Calls

   > 对于一个blocking system call, 比如read(),  当输入没有到来时，此系统调用就会一直blocked在那里，一旦信号发生，则read()会被中断执行， 操作系统转而去执行signal handler。 关键是这个read()系统调用将会fails with error `EINTR` , 这是一个非常有用的特性，比如实现`blocked system call `s timeout`, 但是我们通常需要被信号中断的system call可以重启！以下为两种重启方式：
   >
   > * 手动重启，在每一处system call处，检查其返回值和`errno` , 代码如下:
   >
   > `while((cnt = read(fd, buf, BUF_SIZE)) == -1 && errno == EINTR) continue;` 
   >
   > * 操作系统自动重启， 即通过`sigaction`绑定signal handler时，传入参数：`SA_RESTART`, 注意此参数只是针对每一个signal单独设定的，所以我们可以非常灵活定决定那些signal可以中断系统调用，令其失败，不自动重启；哪些signal中断可以让系统调用自动重启。
   >
   > 但是对于Linux而言，分别规定了哪些system calls可以restart, 哪些不允许restart, 所以`SA_RESTART`并不是都有效。具体详情请参考：`The Linux Programming Interface , Michael Kerrisk, p443-444`

   

   6.  OS Signal Handle  in Rust

      因为信号的平台相关性，比如windows没有采用这种机制， 而Unix/Linux不同版本的Signal机制也有差异， 所以Rust语言没有的语言和标准库中直接提供信号处理封装， 该有第三方库提供，如signal-hook等crate封装提供。

      

      7. 信号处理基本思路：

      > 只是在信号处理函数中做最简单的是， 获取信号信息， 然后离开这个“信号处理流程（异常流）”， 回到“正常的线程执行流”中针对此信号做出更复杂的处理和反应。

      > 简言之， 从受限环境中快速离开， 回到不受限环境中再处理信号； 将所有信号都交给一个专门的线程处理， 比如监听分发。同时将异步信号处理转化为同步信号处理！这样异常流就不会再干扰程序正常执行流了。

      > 将异步转化为同步。

      > 将signal 转化为IO Event , 比如：`Linux signalfd、 timerfd、eventfd `。统一由`select 、poll、 epoll`等高效监听分发， 即是转异步为同步，转signal为IO Event的典范。

      

      

      8. 前人Signal处理模式（态度）收集：

      * Pretending they don’t exist. 视而不见， 不予理睬， 由操作系统默认处理。

      *  Ignore the restrictions and hope for the best.盲目乐观主义， 认为信号发生不频繁， 所以在信号处理函数中随心所欲做事，乐观地认为不会掉坑里。

      * Mask the signals in the whole program and use some mechanism to pull the pending signals out ([sigwait](http://man7.org/linux/man-pages/man3/sigwait.3.html), [signalfd](http://man7.org/linux/man-pages/man2/signalfd.2.html)). The first one needs a dedicated thread, the other one works well with some kind of IO event loop. But it is not applicable solution for a library, because the library has no way to mask signals in all the threads in an application (the application can mask signals before the threads are started and they inherit the mask).  

      * 只做些微不足道的事情，如直接_exit()进程。

      * 只是在信号处理函数中设定一个退出标志， 如一个全局的`bool`变量， 当然这个变量需要`volatile sig_atomit_t `保护，而rust 用`AtomicBool`]护之，避免状态错乱，

        然后每个线程依次检测这个标志，来决定是否退出；前提是每个线程都可以时不时醒过来去检测这个全局标志！现实中，这很难， 有些线程也许在长时间block着。

      * [采用self pipe trick]` https://cr.yp.to/docs/selfpipe.html`

        > `Solution: the self-pipe trick. Maintain a pipe and select for readability on the pipe input. Inside the SIGCHLD handler, write a byte (non-blocking, just in case) to the pipe output. Done.`

      *  采用`Linux signalfd` 将signal处理转化为文件`IO`事件监听分发，如：`select , epoll`，channel等
      * 采用`signal-hook 、ctrlc、mio 、 tokio`等crates显著降低Signal复杂度。

      

      9. How to terminate the current process gracefully!

      > 大概不严谨思路：以signal handler检测退出信号， 方案一：以全局标志通知各个线程，每个线程自己检测标志，而blocked中的thread不能及时有效检测标志。方案二：以多生产者多消费者Channel通知各个线程，结合mio或tokio Event Loop。方案三：以linux signalfd将信号转为文件描述符事件，结合操作系统select 或epoll。二三方案甚好，可及时通知所有线程主动退出，直到main函数return。从而保证内存和资源被释放， 栈对象析构函数被一一调用。
      >
      > 如果调用exit()或abort(),请慎重选择调用点，保证在这点之后，不再有内存和资源没有释放，以及所有栈对象的析构函数都已被调用完毕。Panic 优雅退出思路也大概如此，没有本质区别。
      >
      > 如何尽可能避免线程被长时间blocked, 大概思路：采用异步api, 同步非阻塞api; 采用select , epoll等IO multiplexing 检测可读可写以及signal等，都可以尽可能避免线程长时间阻塞等待！ 对于有些同步阻塞系统api, 如read()之类, 如果必须要调用， 如果其提供了timeout参数最好，一切搞定！如果没有，则可以设定timer signal 在中断系统调用后，不再restart系统调用,  由调用者手动检测此api的返回值：-1和 errno == EINTR ,则判定被signal中断，此后可以主动检测`退出标志` ，然后决定是否再次调用此api。从而有效避免当前线程被长时间阻塞。
      >
      > 对于第三方库其通常都会提供API,用于主动优雅退出阻塞等待， 比如退出xxx.Listen，Run, Wait之类。
      
      

​      

      ##### 10.  我收集整理调试验证了一些Rust Signal 处理的代码例子，希望抛砖引玉。代码目录：`os_signal_handling_in_rust`


​      

      11. About Me
    
      > RUST学习随笔，如有谬误，尽请指正，谢谢。
    
      > 作者：心尘了
      
      > email: [285779289@qq.com](mailto:285779289@qq.com)
      
      > 微信：13718438106
      
      


​      
​      
​      12. Reference List.
​      
      > * The Linux Programming Interface , `Michael Kerrisk`.
      > * Linux多线程服务端编程-使用`muduo C++`网络库， 陈硕.
      > * 深入理解计算机系统，原书第三版，龚奕利，贺莲 译，机械工业出版社.
      > * Linux系统编程，[美]Robert Love著， 人民邮电出版社.
      > * ` https://cr.yp.to/docs/selfpipe.html`
      > * `http://blog.chinaunix.net/uid-28541347-id-5753631.html`
      > * `https://www.ibm.com/developerworks/library/l-reent/index.html`
      > * `https://www.ibm.com/developerworks/cn/linux/l-cn-signalsec/`
      > * `https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html`
      > * `http://www.man7.org/linux/man-pages/man7/signal-safety.7.html`
      > * `https://vorner.github.io/2018/06/28/signal-hook.html`
      > * `https://linoxide.com/linux-how-to/linux-signals-part-1/`
      > * `https://docs.rs/tokio-signal/0.2.7/tokio_signal/`
      > * `https://docs.rs/signal-hook/0.1.6/signal_hook/iterator/struct.Signals.html#examples`
      > * `https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel`
      > * `https://rust-lang-nursery.github.io/cli-wg/in-depth/signals.html`
      > * `https://github.com/tailhook/signal`
      > * `https://www.jianshu.com/p/0cbe7e0c9669`
      > * `https://vorner.github.io/2018/06/28/signal-hook.html`
      > * `http://www.man7.org/linux/man-pages/man7/signal-safety.7.html`
      > * `https://blog.erratasec.com/2018/10/tcpip-sockets-and-sigpipe.html#.XcjCL99fiUk`
      >
> 

12. Thanks All
    
       信息资料繁多，网络信息资源更是浩如烟海，故此难以全部列出引用参考出处，在此一并感谢！如有缺漏，十分抱歉，最后感谢所有前辈的付出和心血。



   













