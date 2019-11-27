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



> RUST学习随笔，如有谬误，尽请指正，谢谢。

> 作者：心尘了

> email: [285779289@qq.com](mailto:285779289@qq.com)

> 微信：13718438106

