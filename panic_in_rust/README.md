# Panic IN Rust

* **Option::None and Result::Err  => unwrap()  expect() to panic.**
* Panicking will unwind the stack, running `destructors` and ensuring that memory is cleaned up. Abort does not do this, and relies on the OS to clean it up properly.
* **Unwinding the Stack or Aborting in Response to a Panic**

> By default, when a panic occurs, the program starts *unwinding*, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately *abort*, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate `[profile] `sections in your `Cargo.toml` file. For example, if you want to abort on panic in release mode, add this:
>
> `[profile.release] panic = 'abort'`

> `RUST_BACKTRACE=1 cargo run` //可以打印出详细栈信息。

> Debug symbols are enabled by default when using `cargo build` or `cargo run` without the `--release` flag

> 编译器提供了一个选项，供用户指定 panic 的实现方式。如下所示：
>
> `rustc -C panic=unwind test.rs  `
>
> `rustc -C panic=abort test.rs`
>
> 如果我们尝试使用 “-C panic=abort” 选项编译代码，可以看到，这个 std::panic::catch_unwind 起不了什么作用。但是，请大家注意，这个 catch_unwind 机制绝对不是设计用于模拟 “try catch” 机制的。请大家永远不要利用这个机制，来做正常的流程控制。它的主要用处在哪里呢，比如下面的这些情况：
>
> 1. 在`FFI`场景下的时候，如果说`C`语言调用了Rust的函数，在`Rust`内部出现了`panic`，如果这个`panic`在`Rust`内部没有处理好，直接扔到C代码中去，会导致C语言产生“`未定义行为（undefined behavior）`”。
> 2. 某些高级抽象机制，需要阻止栈展开，比如`线程池`，如果一个线程中出现了panic，我们希望只把这个线程关闭，而不至于将整个线程池一起被拖下水。



* 异常安全存在4种层次的保证：

1.  No-throw. 这种层次的安全性，保证了所有的异常都在内部正确处理完毕，外部毫无影响。
2.  Strong exception safety. 强异常安全保证，可以保证异常发生的时候，所有的状态都可以“回滚”到初始状 态，不会导致状态不一致问题。
3.  Basic exception safety. 基本异常安全保证，可以保证异常发生的时候，不会导致资源泄漏。
4.  No exception safety. 没有任何异常安全保证。

* forget 函数可以阻止一个对象的析构函数调用。`FFI用`得着！
* In Rust, a panic terminates the `current thread` but is not sent back to and break the` main thread`. if the main thread panics it will terminate all your threads and end your program with code `101`.
* `althought `some panic occurred in child thread , but the panic cannot sent back to and break the parent thread.

> 注意：`以上两点， panic only terminates the current thread, then return a Result::Err(Any) to the  parent thread.`

---

* Catch and Recover Panics

> Note that `panic::catch_unwind` function **may not catch all panics** in Rust. A panic in Rust is not always implemented via unwinding, but can be implemented by aborting the process as well. This function *only* catches unwinding panics, not those that abort the process.
>
> ```rust
> use std::panic;
> 
> fn main() {
> 
>     let result = panic::catch_unwind(|| {
>           println!("no panics , all is ok!");
>        });
>      debug_assert!(result.is_ok());
> 
>      let result = panic::catch_unwind(|| {
>              panic!("oh panic occured !");
>           });
>      debug_assert!(result.is_err());
> 
>      println!("main thread is ok" );
> }
> 
> ```
>
> 

* `Rethrow` Panic

> ```rust
> use std::panic;
> 
> fn main() {
> 
>     let result = panic::catch_unwind(|| {
>         panic!("oh no!, panic occured!");
>     });
> 
>     println!("I am ok 1st", );
> 
>     if let Err(err) = result {
>         println!("I am ok 2nd", );
>         panic::resume_unwind(err);
>         //println!("unreachable here", );
>     }
> 
>     println!("unreachable here", );
> }
> 
> ```
>
> 1. Triggers a panic without invoking the` panic hook`. *注意:不会调用panic hook*
>
> 2. This is designed to be used in conjunction with `catch_unwind` to, for example, carry a panic across a layer of `C` code.
>
>    *主要用于`FFI`, 根据C代码中传出来的Err，在Rust代码中throw a panic.*



* Set a panic hook

>```rust
>use std::panic;
>
>fn main() {
>
>    panic::set_hook(Box::new(|info| {
>        println!("Custom panic hook: {:?}", info);
>    }));
>
>    panic!("Normal panic");
>
>}
>```
>
>Registers a custom panic hook, replacing any that was previously registered.
>
>The panic hook is invoked when a thread panics, but before the panic runtime is invoked. As such, the hook will run with both the aborting and unwinding` runtimes`. The default hook prints a message to standard error and generates a `backtrace` if requested, but this behavior can be customized with the `set_hook` and `take_hook` functions. 
>
>The hook is provided with a `PanicInfo` `struct `which contains information about the origin of the panic, including the payload passed to `panic!` and the source code location from which the panic originated.
>
>***The panic hook is a global resource.***



* Take a panic hook

> ```rust
> use std::panic;
> 
> fn main() {
> 
>     panic::set_hook(Box::new(|_| {
>         println!("Custom panic hook");
>     }));
> 
>     let _ = panic::take_hook();
> 
>     panic!("Normal panic");
> 
> }
> 
> ```
>
> 1. Unregisters the current panic hook, returning it. 注销之前注册的自定义hook.
> 2. If no custom hook is registered, the default hook will be returned.



* Some Rust panic example code

> 1. write panic info to log
>
> ```rust
> use std::panic;
> use std::ops::Deref;
> 
> fn main() {
> 
>  panic::set_hook(Box::new(|panic_info| {
> 
>      let (filename, line) = panic_info.location()
>                                                                      .map(|loc| (loc.file(), loc.line()))
>                                                                      .unwrap_or(("<unknown>", 0));
> 
>      let cause = panic_info.payload()
>                                                  .downcast_ref::<String>()
>                                                  .map(String::deref);
> 
>      let cause = cause.unwrap_or_else(|| {
> 
>          panic_info.payload()
>                                  .downcast_ref::<&str>().map(|s| *s)
>                                  .unwrap_or("<cause unknown>")
> 
>      });
> 
>  println!("Test A panic occurred at {}:{}: {}", filename, line, cause); //you can write panicinfo to log/file/io here.
> 
> }));
> 
>  panic!("oh panic!");
> 
> }
> 
> ```
>
> 2. 统一处理panic
>
> ```rust
> use std::thread;
> use std::panic;
> use std::time;
> use std::any::Any;
> 
> fn main() {
> 
>         println!("Entering main!");
> 
>         let h = thread::spawn(|| {
> 
>             let dur_millis = time::Duration::from_millis(500);
>             thread::sleep(dur_millis);
>             panic!("boom");
> 
>         });
> 
>         let r = h.join();
>         handle(r);
> 
>         let r = panic::catch_unwind(|| {
> 
>             let dur_millis = time::Duration::from_millis(500);
>             thread::sleep(dur_millis);
>             panic!(String::from("boom again!"));
> 
>         });
> 
>         handle(r);
> 
>         println!("Exiting main!");
> 
>     }
> //both of the panic::catch_unwind and thread::spawn  is to return Err(Any)
> //fn handle(r: thread::Result<()>) {
> fn handle<T: std::fmt::Debug>( r:  Result<T, Box<dyn Any + Send + 'static>>) {
> //r: Box<T+ Send + 'static> is an owned pointer to a value (with the original type unknown and dynamically //change) such as std::any::Any , which can be sent across threads and lives as long as the program itself.
> //中文意思大概是： 这是一个指针， 指向动态大小的类型，所指value可以在线程间传递，并且其生命周期长度与本//进程一样长。
>         println!("{:?}", r );
>         match r {
> 
>             Ok(r) => println!("All is well! {:?}", r),
>             Err(e) => {
>                 if let Some(e) = e.downcast_ref::<&'static str>() {
>                     println!("Got an error: {}", e);
>                 }
>                 else if let Some(e) = e.downcast_ref::<String>() {
> 
>                         println!("Got an error: {}", e);
>                 }
>                  else {
>                     println!("Got an unknown error: {:?}", e);
>                 }
>             }
>         }
> 
> }
> ```



* ## What is unwind safety? that is panic safe.

          1. Panic可能引发2个问题

> * A data structure is in a temporarily invalid state when the thread panics.

> * This broken invariant is then later observed.
>
> 简单讲：由于panic发生， 导致某个元素处于无效状态，而且这个无效元素可以被外部引用到！
>
> 反过来讲，只要以上2点同时成立，则必然是unwind not safe。当然就不是panic safe!
>
> Types such as `&mut T` and `&RefCell` are examples which are **not** unwind safe. The general idea is that any mutable state which can be shared across `catch_unwind` is not unwind safe by default. This is because it is very easy to witness a broken invariant outside of `catch_unwind` as the data is simply accessed as usual.
>
> Types like `&Mutex`, however, are unwind safe because they implement poisoning by default. They still allow witnessing a broken invariant, but they already provide their own "speed bumps" to do so.
>
> 共享不可变，可变不共享，按照这个Rust最高哲学原则之一来判定， 通常而言那些`可变且共享`的元素(包括内部可变性)就是不安全的， 故此不满足`UnwindSafe` 。

2. 询问Rust Compiler那些元素是`UnWindSafe`

> ```rust
> use std::cell::RefCell;
> use std::sync::Mutex;
> //do ask rust compiler what types are unwindsafe.
> fn implements<T: std::panic::UnwindSafe>() {}
> 
> fn main() {
> 
>     //可变不共享，共享不可变！
>    //包括内部可变性！
>    //对于可变且共享的元素，可否证明安全？
> 
>    //below all is UnwindSafe.
>     implements::<Option<i32>>();
>     implements::<&Option<i32>>();
>     implements::<&Mutex<i32>>();
> 
> //below all is not UnwindSafe.
>     implements::<&mut i32>();
>     implements::<&RefCell<i32>>();
> 
> }
> ```
>
> 注意：`Mutex`虽然是内部可变且共享元素， 但却是`UnWindSafe`的；当持有这个`Mutex`的线程panic时， 这个`Mutex`通过自身的Poisoned策略， 可以对外部所有线程证明，我中毒了，我被panic毒害了，所以你们可以自己选择是否信任使用我持有的数据！正式因为`Mutex`可以自证清白，所以Rust Compiler认为它是`UnWindSafe`的！由此推到出第3个原则：一个共享可变元素，经历panic后，如果可以对外证明宣称自己已中毒而不会再毒害他人， 则Rust Compiler认为这个元素就是`UnWindSafe` 。此为我的理解，谬误请一笑了之！

> ```rust
> use std::sync::{Arc, Mutex};
> use std::thread;
> 
> fn main() {
> 
>     let lock = Arc::new(Mutex::new(0_u32));
>     let lock2 = lock.clone();
> 
>     let _ = thread::spawn(move || -> () {
>     // This thread will acquire the mutex first, unwrapping the result of
>     // `lock` because the lock has not been poisoned.
>     let _guard = lock2.lock().unwrap();
> 
>     // This panic while holding the lock (`_guard` is in scope) will poison
>     // the mutex.
>         panic!();
>     }).join();
> 
> // The lock is poisoned by this point, but the returned result can be
> // pattern matched on to return the underlying guard on both branches.
>     let mut guard = match lock.lock() {
>         Ok(guard) => guard,
>         Err(poisoned) => poisoned.into_inner(),
>     };
> 
>     println!("{}", *guard );
>     *guard += 1;
>     println!("{}", *guard );
> 
>     assert_eq!(lock.is_poisoned(), true);
>     println!("poisoned: {}",lock.is_poisoned() );
> }
> 
> ```
>
> 3. `AssertUnwindSafe`主动宣称我是`UnWindSafe`的，请Rust Compiler放过我
>
> ```rust
> use std::panic::{self, AssertUnwindSafe};
> 
> fn main() {
> 
> 
>     let mut variable = 4;
>     println!("{}",variable );
>     // This code will not compile because the closure captures `&mut variable`
>     // which is not considered unwind safe by default.
> 
>     // panic::catch_unwind(|| {
>     //     variable += 3;
>     // });
> 
>     // This, however, will compile due to the `AssertUnwindSafe` wrapper
>     let result = panic::catch_unwind(AssertUnwindSafe(|| {
>         variable += 3;
>     }));
> 
>     println!("{}",variable );
>     println!("{:?}",result );
> 
> }
> ```
>
> 4. 采用`C++RAII模式`， 当Panic发生时， 那么在Unwind模式下，Rust保证自动调用每一个`栈对象的析构函数`(但forget主动放弃析构函数被调用的对象除外) ， 从而保证内存和各种资源的有效释放清理。 但是如果是`Abort模式` ， 亦或直接调用了exit()或abort()等系统接口， 则进程当即死亡， 故而Rust 没有自动调用析构函数的机会，内存和资源只能泄露了， 由操作系统打扫战场。



* 代码例子都在目录：panic_in_rust/



* About Me

  > RUST学习随笔，如有谬误，尽请指正，谢谢。

  > 作者：心尘了

  > email: [285779289@qq.com](mailto:285779289@qq.com)

  > 微信：13718438106

  > 

* Reference List.

> * 深入浅出Rust, 范长春著， 机械工业出版社
> * Rust编程之道，张汉东著，电子工业出版社
> * `https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html`
> * `http://rustcc.github.io/rust-by-example/panic.html`
> * `https://stackoverflow.com/questions/27384824/catching-panic-when-rust-called-from-c-ffi-without-spawning-threads`
> * `https://stackoverflow.com/questions/30824258/recovering-from-panic-in-another-thread`
> * `https://stackoverflow.com/questions/26469715/how-do-i-write-a-rust-unit-test-that-ensures-that-a-panic-has-occurred`
> * `https://doc.rust-lang.org/std/panic/fn.catch_unwind.html`
> * `https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md`
> * `https://zhuanlan.zhihu.com/p/53064186`
> * `https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html`
> * `https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html`
> * `https://stackoverflow.com/questions/42456497/stdresultresult-panic-to-log`
> * `https://zhuanlan.zhihu.com/p/24546475`
> * `https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html`
> * `https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.is_poisoned`
> * `https://stackoverflow.com/questions/52300517/is-optioni32-unwind-safe`
> * `https://doc.rust-lang.org/std/process/fn.exit.html`
> * `https://doc.rust-lang.org/std/process/fn.abort.html`
> * 百密一疏，恐有疏漏，在此一并全部致谢！衷心感谢前人的付出和心血。



