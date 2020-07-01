# 											`Pin UnPin 学习笔记`

- 定义

>`A [`Pin`](https://doc.rust-lang.org/nightly/std/pin/struct.Pin.html) `
>
> `ensures that the pointee of any pointer type `P` has a stable location in memory, meaning it cannot be moved elsewhere and its memory cannot be deallocated until it gets dropped. We say that the pointee is "pinned".`
>
>Pin<P>可以确保被任何类型的指针P所指向的目标对象在内存中有一个固定的位置，这意味着它不能被移到别处，并且它的内存在被丢弃之前不能被释放。因此我们说指针指向的目标对象是“固定”的，钉在那里不动。
>
>---
>
>Constructs a new Pin<Box<T>>. If T does not implement Unpin, then T type data will be pinned in memory and unable to be moved.
>
>因为Box作为胖指针用于创建堆对象， 所以Pin<Box<T>> 是典型的Pinning to the heap ! 通俗地讲就是将创建在堆上的data钉在一个固定内存位置，不能移动到其他处。所以Pin可以被看做钉子！钉在墙上不能乱跑。



- 为什么Pin

> 核心诉求是因为Rust要实现`async/await `需要`poll future` , 其实现方法设计一个关键概念：`self-referential structs(自引用结构体)` , 如果不能妥善解决则会引发严重Bug. 
>
> 【`self-referential structs`】是什么？
>
> 说白了就是： 结构体中一个指针` struct field` 指向了本结构体中的另一个` struct field` , 不废话上代码：
>
> ```rust
> #[derive(Debug)]
> struct Test {
>     a: String,
>     b: *const String,
> }
> 
> impl Test {
>     fn new(txt: &str) -> Self {
>         //栈对象
>         Test {
>             a: String::from(txt),
>             b: std::ptr::null(),
>         }
>     }
> 
>     fn init(&mut self) {
>         let self_ref: *const String = &self.a;
>         self.b = self_ref;
>     }
> 
>     fn a(&self) -> &str {
>         &self.a
>     }
> 
>     fn b(&self) -> &String {
>         unsafe {&*(self.b)}
>     }
> }
> 
> //这个代码例子充分说明了`self-referential structs` 引发的严重bug.
> //其实在c/c++中也是一样的， 结构体中一个指针field却指向本结构体中的某个其他field，表面看没什么问题，
> //但是一旦此结构体被move到其他内存位置，如下面代码中的swap, 这种按位copy的方式不会将结构体中的指针更新为新内存位置！
> //这也就是为什么C++提供移动构造函数等语言特性！而Rust语言默认语义是Move, 而且是按位copy方式， 所以出现上面的问题，
> //身在曹营心在汉！其实我觉得C++也只是和稀泥，未能彻底解决！对于变量赋值，对象构造可以用移动构造函数和移动运算符解决；
> //对于底层bit copy/swap这一层面仍然是个漏洞，越过围墙，导致移动构造函数和移动运算符等语言特性成了稻草人！
> //当然C++的语言哲学是：后果自负，作死活该！所以有坑有洞自己看着点！而Rust的语言哲学是：见洞就堵，见坑就填，作死就滚！
> //如何优雅高效彻底解决`self-referential structs` ， Rust语言还在反复不断探讨！旷日持久！
> //这正式我热爱Rust的地方， 对于每一个语言特性都要千锤百炼，绝不滥竽充数， 而且对于发现潜在问题的语言特性零容忍，坚决砍去！
> //所以彻底完整的解决方法还在讨论， 为了实现async/await,  poll a future,  搞个Pin /UnPin就够用了！
> //Pin的意思就是像钉子那样钉在那里不准移动！这样`self-referential structs`的指针问题就没有啦！
> //当然对于`非self-referential structs`的类型标记为UnPin , 即使被Pin也无影响，照样Move, 因为不存在自引用，完全安全呀！
> fn main() {
>     let mut test1 = Test::new("test1");
>     test1.init();
>     let mut test2 = Test::new("test2");
>     test2.init();
> 
>     println!("a: {}, b: {}", test1.a(), test1.b());
>     std::mem::swap(&mut test1, &mut test2); //按bit 直接复制交换。
>     test1.a = "I've totally changed now!".to_string();
>     println!("a: {}, b: {}", test2.a(), test2.b()); //证明指针test2.b仍然指向test1.a位置，但是他应该指向test2.a才对的！！！
> 
> }
> ```
>
> 详细学习，我推荐好文章：`https://zhuanlan.zhihu.com/p/67803708`    , `https://www.jianshu.com/p/8152b9fb8161`
> `https://cfsamson.github.io/books-futures-explained/4_pin.html`



- `栈和堆`

> `http://www.rust-compare.com/site/stack_heap.html` 这篇文章短小清晰！可以很好帮助体会栈和堆， 如：
>
> ```rust
> {
>     // single char allocated on the stack 创建在栈上， a是个栈变量，同时其中存储`a` 。
>     let a: char = 'a'; // char属于rust基础类型实现了Copy trait, 所以执行复制语义，而非Move语义。
> 
>     // single char allocated on the heap 创建在堆上， b是个栈变量(胖指针)， 而其指向的data: `b` 则创建存储在堆上。
>     let b = Box::new('b'); //Box类型执行Move语义！
> 
>     // array of 23 chars allocated on the heap 数组创建在堆上。
>     // char pointer allocated on the stack  c是一个Box类型胖指针创建在栈上， 而其指向的data: ['c', 23]则创建存储在堆上。
>     let c = Box::new(['c'; 23]);
> 
>     // heap allocated memory is freed when the
>     // variable goes out of scope
>     // however, you can force freeing by using
>     // the mem::drop method
>     //Box会在析构时自动释放堆上的data, 不过可以通过std::mem::drop手动提前释放。
>     mem::drop(b); 
>     mem::drop(c);
> }
> ```
>
> 对于没有实现Copy Trait的类型， Rust默认一律执行Move 语义！既是传递所有权， 比如let a = Box::new('c') ;  let b = a; 则表示a放弃所有权， 而b接管a的所有权！同时a彻底失效，不能再被访问！所有权代表意思是： 不管data创建存储在栈上，还是堆上！现在它完全由你负责， 你生它生， 你死它死！你是它的主人。
>
> ---
>
> 注意：变量和值并不一定存储在一起， 对于简单的基础类型，如： `数值，字符，bool`等变量和值是在一起，同在一块栈内存； 对于类似Box类型的变量通常是创建存储在栈上， 其值（数据对象）则创建存储在堆上！
>
> 1. 对于同一个类型， 只要实现Copy trait 则必须一同实现Clone trait， 反之则否。
>
> 2. 对于同一个类型， Copy trait 和 Drop trait 不允许同时实现。
>
> 3. 我们可以把Rust Move语义想象成执行了一个`memcpy`。 比如： 比如let a = Box::new('c') ;  let b = a;  见下文解释： 
>
>    
>
>    variable on stack :  a[`0x110`]     --->  value on heap:  `0x110`['c']
>
>    对a `memcpy/swap` 就是按位读出栈变量a的内容：`0x110` ,  复制给b , b[`0x110`]  , 现在b也指向了堆`0x110`['c']， 也可以说接管了a的所有权， 而a放弃了所有权，从此这块堆内存：`0x110`['c'] 与b 同生共死！而a则一边凉快去了！没他事了。
>
> 我之所以这么啰嗦， 就是因为大家在此处容易迷糊， 明明是Move 怎么还是`按bit memcpy/swap`呢？？？？？
>
> 所以C++guys 一听Move 语义底层需要`memcpy` 就误认为效率低下， 譬如`Box 和Vec数组`等， 以为就是C++的值传递呢！ 如果真实整个数组元素都要复制， 当然效率堪忧啦！ 但是非也， `Vec数组`和Box一样都是胖指针， 按位`memcpy/swap`的是胖指针自身， 无关其指向的堆上data对象！再说白点：rust 按位`memcpy/swap`的是智能指针变量自身，而非其所指向的堆上data对象。 也可以理解为：按`bit memcpy`的是链表header块， 而非整个链表。























- Reference

> `https://ehsanmkermani.com/2019/08/16/rust-std-study-series-pin/`
> `https://doc.rust-lang.org/std/pin/index.html`
> `https://cfsamson.github.io/books-futures-explained/4_pin.html`
> `https://rust-lang.github.io/async-book/04_pinning/01_chapter.html`
>
> `https://zhuanlan.zhihu.com/p/67803708`
> `https://www.jianshu.com/p/8152b9fb8161`
>
> `https://doc.rust-lang.org/std/boxed/struct.Box.html#method.pin`
>
> `https://doc.rust-lang.org/std/mem/fn.swap.html`
>
> `http://www.rust-compare.com/site/stack_heap.html`
>
> `https://stackoverflow.com/questions/30288782/what-are-move-semantics-in-rust`
> `https://rufflewind.com/2017-02-15/rust-move-copy-borrow#:~:text=In%20Rust%2C%20any%20variable%20whose%20type%20does%20not,in%20a%20copy%2C%20as%20shown%20by%20the%20bifurcation.`
> `https://www.codevamping.com/2018/12/rust-move-copy/`
> `https://doc.rust-lang.org/rust-by-example/scope/move.html`
>
> `https://stackoverflow.com/questions/29490670/how-does-rust-provide-move-semantics    `
>
> `https://stackoverflow.com/questions/51704063/why-does-rust-not-allow-the-copy-and-drop-traits-on-one-type`
>
> `https://doc.rust-lang.org/std/marker/trait.Copy.html`
>
> `https://doc.rust-lang.org/std/boxed/struct.Box.html#method.pin`
>
> `https://dev.to/werner/move-semantics-vs-copy-semantics-pkb`
>
> 《深入浅出Rust》范长春著， 机械工业出版社