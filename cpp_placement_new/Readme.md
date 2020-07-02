# 				`C++ Placement new in Rust Tracking `







- 相关资料收集杂记

> 
>
>
> Rust中的堆分配现在是如何工作的???
>
> 堆分配隐藏在Rust stable中的Box类型后面。当您在一个Box类型中实例化数据时，您将把数据放入堆中，并且Box类型存储一个指向该堆分配数据的内部指针。
>
> 而且堆内存的分配和释放完全由Rust 编译器负责！隐藏在Rust 编译器背后！
>
> 现在Rust stable 还不支持C++ placement new ， 相关的讨论争论还在继续！ 我认为Rust安全的基石在于对内存的严格控制！ 如果贸然在围墙上开一个洞， 我看还需谨慎考虑和缜密设计！任何新语言特性的加入都不允许危害Rust的安全基石！
>
> 相关的讨论和设计，请看:
>
> http://blakesmith.me/2018/12/31/what-is-placement-new-in-rust.html
>
> https://github.com/rust-lang/rust/issues/27779
> https://github.com/rust-lang/rust/pull/48333
> https://internals.rust-lang.org/t/removal-of-all-unstable-placement-features/7223



> Rust每一新的语言特性的加入都要经过旷日持久的讨论和争论， 即使已经加入的语言特性若发现严重问题，也可能砍去！这正是Rust高质量高效率的基石，
>
> 所以C++ placement new还在讨论完善之中， 我会持续关注更新。



- Author

> 学习随笔，如有谬误，望请海涵雅正，谢谢。
>
> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)
>
> git: https://github.com/yujinliang