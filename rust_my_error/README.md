# 自定义Error in Rust

我参考了`std::io::Error`的实现代码，写了两个代码小例子，我尽可能简化之，只保留必须实现的Trait.

对于Rust的错误处理， Option<T>可以表达`有和无`， Result<T,E>进而表达`对和错`， 很多时候，知道出现错误了，就够了！ 但是有时候我们必须针对每一种具体错误类型做出更加有针对性的处理逻辑！所以第三个要求：光知道出错还不够，还要知道具体发生了什么种类错误！特别是对于crate库的实现者，对于返回给caller的Result::Err必须给出具体明确的错误类型，这样可以令caller可以针对性处理。

C语言大家基本都学过， C函数返回不同的有符号整数,用于表示不同的错误种类！`Unix/Linux`的 `errno` 也是如此！

很朴素，很直接，但是可读性不高！对于Rust而言，直接在Result::Err(错误码)包含有符号整数错误码当然很简单！但是Rust Style呢？通常是自定义Error，关键点：

* `pub enum MyError   { ...}` 或`pub struct MyError{}`
* `impl trait Debug,Display for MyError`
* `impl trait std::error::Error for MyError`
* `impl trait From<> for MyError`

> 提示： trait From用于将不同模块或crate的Error转化为我的自定义Error; 而且Rust 要求`自定义Error`必须实现`std::error::Error`trait , 且这个`trait` 中，只有`fn source`必须要实现， `fn description`  只要你实现了`trait Display` 就没必要实现了，`fn cause`淘汰了，我在`rust 1.39`中实验如此。
>
> 具体还是看代码例子吧。
>
> 在实现过程中，必须注意的问题：
>
> 1. 各种元素：Sized, 即必须是大小固定可知的, 编译期常量。
> 2. 满足 Trait Send , Sync限定，因为可能在线程间传递共享。
>

`以上个人理解恐有谬误，尽请指正！`

------

#    [Rust: Structuring and handling errors in 2020](https://nick.groenen.me/posts/rust-error-handling/)-	学习笔记						

`直接上菜， 首先介绍两个新的Crate用于Rust Error处理，anyhow 和thiserror, 其代表Rust Error处理的最新探索成果! anyhow面向Application应用程序开发， thiserror面向Library开发， 为什么一个Rust Error处理需要分成两个Crate来处理呢？！` 理由如下：

> (1) 库应专注于生成有意义的结构化错误类型/变体。这允许应用程序轻松区分各种错误情况。
>
> (2) 应用程序主要消化错误。
>
> (3) 库需要将错误从一种类型转换为另一种类型。比如：将底层IO错误封装进本库提供的高级错误，这样调用者才能区分此底层IO错误具体来自于那个库。
>
> (4) 库在更改错误或创建新错误时必须小心，因为这些错误很容易为调用者引入破坏性更改。所以要求库可以更容易地更改和新建错误，同时尽可能避免祸及调用者。
>
> (5) 库返回错误，而应用程序需要决定这些错误是否格式化以及如何显示给人。
>
> (6) 应用程序可能还希望分析和检查错误，例如将错误转发到异常跟踪系统，或者在认为安全时重试操作。
>
> (7) 库的`pub API`应该始终使用std::Result和自定义Error 类型(并且它impl了std::error::Error Trait) ，而failure::Fail这类Rust Error处理Crate没有能与Rust原生的Error处理很好地融合， 独自创造了一套，也难以很好的和用户代码相融合， 增加了用户学习负担。
>
> 总结： 基于以上理由， 将错误处理分为两个crate就非常必要了！同时提炼出优质Rust Error处理crate的特性： A. 可以与Rust原生Error处理很好融合。B. 可以与用户代码很好地融合，最好是不漏踪迹，不污染用户代码。 C. 极大地降低学习和使用负担，最好零负担。 D. 非常容易更改和新建错误，不会导致调用者破坏性修改代码。 E. 可以backtrace, 清晰表达错误类型、源头和传播路径。 F.  可以给每个错误以人性化的可读信息。G. 灵活有效的错误分析、检查、转发、重试机制。
>
> 【详细内容请看】：`https://nick.groenen.me/posts/rust-error-handling/`



> 参照原文，我整理调试出两个代码例子， `thiserror_t 和anyhow_t `, 放到了`https://github.com/yujinliang/rust_learn/tree/master/rust_my_error` ， 有兴趣可以看看。



- Author


> RUST学习随笔，如有谬误，尽请指正，谢谢。
>
> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)



- Reference:

`https://nick.groenen.me/posts/rust-error-handling/`

`https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html`

`https://mozillazg.com/2017/10/rust-cargo-use-local-crate-package.html`

`https://doc.rust-lang.org/rust-by-example/cargo/deps.html`