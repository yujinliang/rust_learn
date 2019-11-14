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



