# `A Guide to Global Data in Rust`-学习笔记

- 为什么需要Global Data

> (1) 应用程序的配置。
>
> (2) 例如全局变量，不需要一层层函数传参。
>
> (3) 数据库链接，连接池以及其他网络资源。
>
> (4) 日志。
>
> (5) 例如在`build.rs`中生成Rust代码，如：date/time/git commit rev.



- 权衡取舍

> (1) 编译时/运行时 load global data value.
>
> (2) 只读/可写。
>
> (3) 生命周期 : 要'static , 还是局部一点。
>
> (4) 创建到栈上， 还是堆上。
>
> (5) 并发读写否。



- 多种方案比较

> (1) rust let关键字，在`fn main()`中定义，然后通过函数参数传递。缺点：就是麻烦，需要层层函数传参。
>
> (2) `rust const`关键字，其生命周期为'static。 缺点：不可变， 而且限制为编译器常量和`const fn `.
>
> (3) `lazy_static`crate and `once_cell` crate,  缺点：数据对象必须满足Sync trait, 以及进程退出时，数据对象的析构函数不被执行，所以不要指望通过其析构函数清理资源。
>
> (4)`arc-swap`crate 解决并发读写Global Data的问题。
>
> (5) `std::include!`、`std::include_str!`、`std::include_bytes!`， 将一段Rust源码复制拼接到你的Rust源码文件中。
>
> 更多详情请看：`https://github.com/paulkernfeld/global-data-in-rust` ， 我就不再传话了。



- 后记

  任何程序，任何编程语言, 对于`Global Data`的用法都是一个非常重要的关切点！一方面关乎语言哲学， 另一方面关乎实际工程代码的耦合度！所以恰当使用`Global Data`是极其重要的！代码过度耦合则难于管理/扩展/维护！所以非常有必要反复强调！这篇笔记只是抛砖引玉，希望您能阅读原文， 深入学习`Global Data`的使用指导。`https://github.com/paulkernfeld/global-data-in-rust`
  
  > 程序开发需要始终贯彻`依赖倒置原则`， 封装隔离变化， 抽象出稳定接口！即抽象接口，具化实现！而耦合代表依赖，依赖稳定的抽象层，如此代码耦合才能适度。



- Author

> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)
>
> git：https://github.com/yujinliang/rust_learn
>
>  `水平有限，笔记草乱，如有谬误，尽请指正！`



- Reference

> `https://github.com/paulkernfeld/global-data-in-rust`