# 										Rust异步编程杂记









- 异步编程大敌阻塞

> `https://blog.hwc.io/posts/rust-futures-threadsleep-and-blocking-calls-inside-async-fn/`
>
> `https://stjepang.github.io/2019/12/04/blocking-inside-async-code.html`
>
> `https://rickyhan.com/jekyll/update/2019/12/22/convert-to-async-rust.html`
>
> `https://async.rs/blog/stop-worrying-about-blocking-the-new-async-std-runtime/`
>
> ---
>
> 对于异步编程， 我认为async runtime应该自动检测blocking发生， 一旦达到时间阀值， 自动为async task executor 创建新的执行线程， 从而避免队列中其他aysnc task因为没有线程来分配而不能运行！从而无缝地将同步阻塞和异步编程很好地合二为一！这样两个世界变为一个世界！这样，我们就可以很自然地使用diesel/rayon等计算密集型的库来真正解决问题。比较彻底地解决了异步编程只适合IO密集型项目的缺点！
>
> 其实一切的焦点在于效率： golang goroutine采用抢占式调度了，所以阻塞不再是问题！而Rust 协程是协作式调度， 一旦发生阻塞，则协作无法完成，大家都无法再继续执行下去！前者可以彻底避免阻塞的危害！但是抢占是有代价的！牺牲一定的性能！ 而后者没有各种检测和抢占代码效率损耗，执行效率可能会更优！但是无法克服阻塞的问题！



- 对于不能直接借用本地栈变量的克服方法：

> 1. move 闭包， 且clone要借用的变量。
>
> 2. 不要borrow， 而是move ownship.
> 3. Task::scop.
> 4. Task::Block_on
> 5. Arc.
> 6. Pin
> 7. static variable
> 8. impl Future
>
> 具体例子请看：`https://github.com/yujinliang/rust_learn/tree/master/async_std/how_to_ref_self`





- 协作式调度 vs 抢占式调度(preemptive - cooperative)

> `https://www.zhihu.com/question/271657274/answer/376416028`
>
> 【抢占方法】
>
> (1). os signal
>
> 这个提案要实现的，如字面意思，就是强行让一个 goroutine 让出 CPU，不管该 goroutine 在做什么，不需要 goroutine 的“协作”，就能抢占该 goroutine 的 CPU 时间。go 现在的调度器，如果想从外部让一个 goroutine 让出 CPU 时间，只能在函数的入口处做一些手脚，让该 goroutine 在调用函数之前，发现它应该让出 CPU，这就是协作式的，因为需要 goroutine 执行到那一个路径，外部只能等待它执行到那里，或者其他一些触发到调度的代码路径。
>
> 这个提案的方案是，直接用信号让执行 goroutine 的系统线程切换到信号处理器，从而实现 goroutine 的打断。
>
> (2).  在每个函数调用入口加入调度代码，判断是否抢占！ 不彻底，因为一旦进入函数再阻塞， 则无法抢占！而且效率代价大！





- Author

> 学习随笔，如有谬误，望请海涵雅正，谢谢。
>
> 作者：灵山行者
>
> email: [285779289@qq.com](mailto:285779289@qq.com)
>
> git: https://github.com/yujinliang







- Reference

> `https://blog.logrocket.com/a-practical-guide-to-async-in-rust/`
>
> `https://alastairreid.github.io/rust-verification-tools/`
>
> `https://github.com/rust-lang/async-book`
>
> `http://xion.io/tag/async.html`
>
> `https://omarabid.com/async-rust`
>
> `https://async.rs/`
>
> `https://areweasyncyet.rs/`
>
> `https://zhuanlan.zhihu.com/p/37209852`
>
> `https://blog.rust-lang.org/2019/11/07/Async-await-stable.html`
>
> `https://rust-lang.github.io/async-book/03_async_await/01_chapter.html`
>
> `https://www.philipdaniels.com/blog/2019/async-std-demo1/`
>
> `https://book.async.rs/concepts/tasks.html`
>
> `https://rust-lang.github.io/async-book/`

