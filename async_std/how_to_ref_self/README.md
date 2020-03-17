## Rust `async` block-future-task::spawn-how to ref self

- 综述

rust有借用的概念，代码中具体称为引用，如：`&str,  &mut String,  &'a str,  &'b mut String`, 而且每一个变量都有一个关键属性：`life time生命周期`，代表每一个变量从创建到销毁的全过程！引用变量（可理解为c语言中指针变量）同样有自己的生命周期！通俗地讲就是存活时间，通常由变量所在的词法作用域决定！而且每一个引用具有一个关键参数属性：`生命周期参数`， 别糊涂，这个`生命周期参数`是用于限定描述其可以指向的变量的生命周期， 因为rust 借用检查器强制要求，每一个引用（指针）变量只可以指向比自己命长的变量（大于等于）， 否则编译器拒绝！

对于一个函数/代码块内，通常不太需要`生命周期参数`， 因为编译器容易分析判断出每一变量的生命长短和引用的有效性，不太需要认为干预！但是对于`函数传参（引用传递）、闭包捕获外部变量， async fn/block`等等需要跨越不同作用域来分析每一个引用的有效性合法性就比较复杂了， 此时rust编译器需要为引用人工标注`生命周期参数` ， 这可能会形成一个引用的传递链条， 编译器顺藤摸瓜分别检查每一个引用变量自身的存活时间，进而再检查其所指向目标变量的存活时间，只要满足`短命者可以指向长命者，反之拒绝`这条铁律，就可以避免`悬空指针`问题。

在实际编码中，特别是对于`闭包、异步async/await, future`等等的编码中， 很容易出现在类成员函数中，self引用需要被传递的需求！Rust编译器倔强地报错，`IDE`一片红，搞了半天就是过不去，非常有挫败感！所以我花了点时间总结了几种self引用传递的方法，水平有限精力不足，难免谬误，权当抛砖引玉，还请诸君见谅！



- 引用计数智能指针

```rust
use async_std::task;
use async_std::sync::Arc;

struct Circle {
    radius:f64
}
impl Circle {
    async fn area(self:Arc<Self>) -> f64 {
        let self_divide = self.clone();
        let join_handle = task::spawn(async move {
            2.0*3.1415*self_divide.radius 
        });
        join_handle.await
    }

}
fn main() {
    task::block_on(async {
        let by_arc = Arc::new(Circle{radius:30.5});
        println!("{}", by_arc.area().await);
    })
}
```



- block_on

```rust
use async_std::task;

struct Circle {
    radius:f64
}
impl Circle {
    async fn area(&self) -> f64 {
        task::block_on(async {
            2.0*3.1415*self.radius 
        }) 
    }

}
fn main() {
    task::block_on(async {
        let by_blockon = Circle{radius:30.5};
        println!("{}", by_blockon.area().await);
    })
}
```



- static variable

```rust
use async_std::task;

struct Circle {
    radius:f64
}
impl Circle {
    async fn area(&'static self) -> f64 {
       let j_handle = task::spawn(async move {
            2.0*3.1415*self.radius 
        }) ;
        j_handle.await
    }

}
fn main() {
    task::block_on(async {
        static BY_STATIC:Circle = Circle{radius:30.5};
        println!("{}", BY_STATIC.area().await);
        println!("{}", BY_STATIC.area().await);
    })
}
```



- `impl Future`

```rust
use async_std::task;
//use async_std::task::JoinHandle;
use futures::{future, Future,FutureExt};
use std::error::Error;

struct Circle {
    radius:f64
}

impl Circle {
   async  fn make_a_future<'a>(&'a self) -> impl Future<Output=f64> +'a {
        //below code compilation failed, coz ref self, unless add move for async block.
        async  move {
            2.0*3.1415*self.radius 
        }
    }

    async  fn make_b_future<'a>(&'a self) -> impl Future<Output=f64> +'a {
        future::ok(self).map(|pa:Result<&'a Self, Box<dyn Error+Send>>| {2.0*3.1415*pa.unwrap().radius})
     }

}
fn main() {
    task::block_on(async {
        let by_life = Circle{radius:30.5};
        //1.for task::spawn
        //compilation failed with task::spawn.
       //let r = task::spawn(by_life.make_a_future()).await;

       //2.for async block
       let ra = by_life.make_a_future().await;
        println!("{:?}", ra.await);
        
      //3.for future
        let rb = by_life.make_b_future().await;
        println!("{:?}", rb.await);
    })
}
```

> ```rust
> pub fn spawn<F, T>(future: F) -> JoinHandle<T> where
>     F: Future<Output = T> + Send + 'static,
>     T: Send + 'static, 
> //https://docs.rs/async-std/1.5.0/async_std/task/fn.spawn.html
> //tokio和async-std中， task::spawn基本相同， 基本都施加了`static`限制， 所以rust 编译器报错，
> //希望self引用的是static类型的变量.
> //我理解：一个executor什么时候调度执行task是不确定的， 也许立刻，也许未来， 所以要求task要活的足够长！
> //故此要求其内部引用指向的变量要活得和自己一样长！
> //我在想，对于detached task by task::spawn这一情况，static确实需要！但是对于JoinHandle类型task::spawn这一情况， 当前函数代码块持有JoinHanle.await, 那么对于self的严格static要求是否可以视情况而定放宽一点？！因为JoinHandle.await block当前代码块不会死掉，一直等待task被调度执行完毕返回！当然这需要rust编译器和crate更紧密配合，不容易呀！所以我认为可能有改进空间，并未详细考证， 粗鄙乱议而已！
> ```

- 总结

  本人水平和精力有限，目前只是整理了以上4中方法， 只做抛砖引玉，希望高人指点收集更多好方法！上面给出的第一个方法为通用方法， 大家都用！官方也是这么用！所以推荐给大家！其后的方法或多或少都有一定的局限性，适用面窄！只当拓宽思路！再啰嗦两句，Rust异步编程的引入明显加大了`生命周期`分析的难度！Rust语言自身及其`tokio/async-std/future`等crates也在逐步成熟完善之中，易用性也在不断提高！不断求索吧！加油！

>  代码仓库:`https://github.com/yujinliang/rust_learn/tree/master/async_std/how_to_ref_self`



- Reference

https://stackoverflow.com/questions/48613350/how-to-return-a-future-combinator-with-self

https://docs.rs/futures/0.3.4/futures/future/trait.FutureExt.html#method.then

https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

https://docs.rs/async-std/1.5.0/async_std/task/fn.spawn.html

