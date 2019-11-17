# Rust Option<T> and Result<T,E> 错误处理

* ? 如何向外传递错误

> ```rust
> use std::io;
> use std::io::Read;
> use std::fs::File;
> 
> fn read_username_from_file() -> Result<String, io::Error> {
> 
>     let f = File::open("hello.txt");
> 
>     let mut f = match f { //此处match就是?的实现方法。
>         Ok(file) => file,
>         Err(e) => return Err(e),
>     };
> 
>     let mut s = String::new();
> 
>     match f.read_to_string(&mut s) {
>         Ok(_) => Ok(s),
>         Err(e) => Err(e),
>     }
> }
> ```
>
> 注意： 第一个match的逻辑就是问号?的实现，问号？只是一个语法糖。如果`Ok`,则返回`Ok`内包含的Value; 如果Err, 则直接返回此Err. [**The** **?** **Operator Can Only Be Used in Functions That Return** **Result**](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#the--operator-can-only-be-used-in-functions-that-return-result)



* 对待Option<T> 剥洋葱, Option可以表达有和无的问题。

> ```rust
> //unwrap()  剥洋葱取出Some内的Value.
> let x = Some("air");
> assert_eq!(x.unwrap(), "air");
> //unwrap()遇到None则panic.
> let x: Option<&str> = None;
> assert_eq!(x.unwrap(), "air"); // fails
> ```
>
> ```rust
> //unwrap_or 剥洋葱取出Some内的Value.
> assert_eq!(Some("car").unwrap_or("bike"), "car");
> //unwrap_or遇到None则返回_or中指定的默认值.
> assert_eq!(None.unwrap_or("bike"), "bike");
> ```
>
> ```rust
> let k = 10;
> //unwrap_or_else 剥洋葱取出Some内的Value.
> assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
> //unwrap_or_else遇到None则返回_or_else中指定闭包计算的值。
> assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
> ```
>
> ```rust
> //unwrap_none 剥洋葱取出Some内的Value；如果遇到None,则returning nothing.
> #![feature(option_unwrap_none)]
> 
> use std::collections::HashMap;
> let mut squares = HashMap::new();
> for i in -10..=10 {
>  // This will not panic, since all keys are unique.
>  squares.insert(i, i * i).unwrap_none();
> }
> ```
>
> ```rust
> //unwrap_or_default 剥洋葱取出Some内的Value；如果遇到None则返回此类型的初始默认值，比如：
> //Option<i32> ,则i32的默认值为:0
> let x:Option<i32> = None;
> assert_eq!(x.unwrap_or_default(),0 );
> ```
>
> ```rust
> //convert Option::Some("foo") to Result::Ok("foo")
> //将Option<T>转化为Result<T,E>
> let x = Some("foo");
> assert_eq!(x.ok_or(0), Ok("foo"));
> //pub fn ok_or<E>(self, err: E) -> Result<T, E>
> //如果None, 则转化为Result::Err, 而Err中包含的值，就是ok_or()的实参。
> let x: Option<&str> = None;
> assert_eq!(x.ok_or(0), Err(0));
> ```
>
> ```rust
> //convert Option::Some("foo") to Result::Ok("foo")
> //将Option<T>转化为Result<T,E>
> let x = Some("foo");
> assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
> //pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
> //where
> //F: FnOnce() -> E, 
> //如果None, 则转化为Result::Err, 而Err中包含的值，就是ok_or_else实参指定的闭包返回的值。
> let x: Option<&str> = None;
> assert_eq!(x.ok_or_else(|| 0), Err(0));
> ```
>
> ```rust
> //Unwraps an option, yielding the content of a Some.
> //Panics if the value is a None with a custom panic message provided by msg.
> 
> let x = Some("value");
> assert_eq!(x.expect("the world is ending"), "value");
> 
> let x: Option<&str> = None;
> x.expect("the world is ending"); // panics with `the world is ending`
> ```
>
> ```rust
> //Panics if the value is a Some, with a panic message including the passed message, and the content of the //Some.
> //Unwraps an option, expecting None and returning nothing.
> 
> #![feature(option_expect_none)]
> 
> use std::collections::HashMap;
> let mut squares = HashMap::new();
> for i in -10..=10 {
>     // This will not panic, since all keys are unique.
>     squares.insert(i, i * i).expect_none("duplicate key");
> }
> 
> #![feature(option_expect_none)]
> 
> use std::collections::HashMap;
> let mut sqrts = HashMap::new();
> for i in -10..=10 {
>     // This will panic, since both negative and positive `i` will
>     // insert the same `i * i` key, returning the old `Some(i)`.
>     sqrts.insert(i * i, i).expect_none("duplicate key");
> }
> ```
>
> 

* Option的Combinator组合子

> ```rust
> //x.and(y) ,if x is None then return None, 否则 return y.
> 
> let x = Some(2);
> let y: Option<&str> = None;
> assert_eq!(x.and(y), None);
> 
> let x: Option<u32> = None;
> let y = Some("foo");
> assert_eq!(x.and(y), None);
> 
> let x = Some(2);
> let y = Some("foo");
> assert_eq!(x.and(y), Some("foo"));
> 
> let x: Option<u32> = None;
> let y: Option<&str> = None;
> assert_eq!(x.and(y), None);
> ```
>
> ```rust
> //Returns None if the option is None, otherwise calls f with the wrapped value and returns the result.
> //x.and_then(y_closure), if x is None then return None, 否则调用闭包y_closure,将其执行结果返回。
> //注意闭包y_closure的实参就是x内部包含的值，比如Some(2) , 则x=2
> 
> fn sq(x: u32) -> Option<u32> { Some(x * x) }
> fn nope(_: u32) -> Option<u32> { None }
> 
> assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16)); 
> assert_eq!(Some(2).and_then(sq).and_then(nope), None);
> assert_eq!(Some(2).and_then(nope).and_then(sq), None);
> assert_eq!(None.and_then(sq).and_then(sq), None);
> ```
>
> ```rust
> //x.or(y) if x is not  None then return x; 否则返回y
> 
> let x = Some(2);
> let y = None;
> assert_eq!(x.or(y), Some(2));
> 
> let x = None;
> let y = Some(100);
> assert_eq!(x.or(y), Some(100));
> 
> let x = Some(2);
> let y = Some(100);
> assert_eq!(x.or(y), Some(2));
> 
> let x: Option<u32> = None;
> let y = None;
> assert_eq!(x.or(y), None);
> ```
>
> ```rust
> //x.or_else(y_closure) if x is not None then return x; 否则调用闭包y_closure，返回闭包执行结果。
> 
> fn nobody() -> Option<&'static str> { None }
> fn vikings() -> Option<&'static str> { Some("vikings") }
> 
> assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
> assert_eq!(None.or_else(vikings), Some("vikings"));
> assert_eq!(None.or_else(nobody), None);
> ```
>
> ```rust
> fn main() {
> 
> //x.xor(y) 如果 x and y 都是None则结果返回None
> //如果x and y都是Some ,则不论Some中包含的值是否相同相等， 结果返回None
> //如果x is not None and y is None , 则结果返回x
> //如果x is None and y is not None , 则结果返回y
>  
> let x = Some(2);
> let y: Option<u32> = None;
> assert_eq!(x.xor(y), Some(2));
> 
> let x: Option<u32> = None;
> let y = Some(2);
> assert_eq!(x.xor(y), Some(2));
> 
> let x = Some(2);
> let y = Some(2);
> assert_eq!(x.xor(y), None);
> 
> let x = Some(3);
> let y = Some(2);
> assert_eq!(x.xor(y), None);
> 
> let x = Some(2);
> let y = Some(3);
> assert_eq!(x.xor(y), None);
> 
> let x: Option<u32> = None;
> let y: Option<u32> = None;
> assert_eq!(x.xor(y), None);
> 
> }
> ```
>
> ```rust
> //x.filter(y_closure) 如果x is None then return None.
> //如果x is not None 则调用闭包y_closure, 如果闭包执行结果为：false , 则return None.
> //如果闭包执行结果为true, 则return x
> 
> fn is_even(n: &i32) -> bool {
>  n % 2 == 0
> }
> 
> assert_eq!(None.filter(is_even), None);
> assert_eq!(Some(3).filter(is_even), None);
> assert_eq!(Some(4).filter(is_even), Some(4));
> ```
>
> ```rust
> //x.map(y_closure) 以x为实参调用闭包y_closure， 返回闭包执行结果，形如：Some(z) ，z为闭包计算结果值。
> //convert Option<T> to Option<U>
> 
> fn main() {
> 
>  let maybe_some_string = Some(String::from("Hello, World!"));
>  // `Option::map` takes self *by value*, consuming `maybe_some_string`
>  let maybe_some_len = maybe_some_string.map(|s| s.len());
>  assert_eq!(maybe_some_len, Some(13));
> 
>  //x.map(y_closure) 如果x is None , then return None.
>  let t_none: Option<i32> = None;
>  let none_map = t_none.map(|_x|  3 );
>  assert_eq!(none_map, None);
> 
>  //x.map(y_closure) 如果x is None , then return None.
>  let t_2none: Option<i32> = None;
>  let none_none_map :std::option::Option<std::option::Option<i32>> = t_2none.map(|_x|  None);
>  assert_eq!(none_none_map, None);
>  
>  //x.map(y_closure) 如果x is not None , 则调用闭包y_closure，闭包结果为：None， 最终返回结果
>  //为:Some(None)
>  let closure_r_none: std::option::Option<std::option::Option<i32>>  = Some(6).map(|_x|  None );
>  assert_eq!(closure_r_none, Some(None));
>  
> }
> ```
>
> ```rust
> //x.map_or( y_default, z_closure) if x is not None , 则以x为实参调用闭包z_closure，并返回结果
> //if x is None then return y_default .
> //注意：结果没有用Some包裹。
> let x = Some("foo");
> assert_eq!(x.map_or(42, |v| v.len()), 3);
> 
> let x: Option<&str> = None;
> assert_eq!(x.map_or(42, |v| v.len()), 42);
> ```
>
> ```rust
> //同上， 唯一不同是：默认值为直接指定改为由闭包计算而得。
> let k = 21;
> 
> let x = Some("foo");
> assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
> 
> let x: Option<&str> = None;
> assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
> ```
>
> ```rust
> //Converts from Option<T> (or &Option<T>) to Option<&T::Target>.
> //注意原来的Option不变， 而是新建一个Option<&T::Target>
> #![feature(inner_deref)]
> 
> let x: Option<String> = Some("hey".to_owned());
> assert_eq!(x.as_deref(), Some("hey"));
> 
> let x: Option<String> = None;
> assert_eq!(x.as_deref(), None);
> ```
>
> ```rust
> //Converts from Option<T> (or &mut Option<T>) to Option<&mut T::Target>.
> //注意原来的Option不变， 而是新建一个Option<&mut T::Target>
> #![feature(inner_deref)]
> 
> let mut x: Option<String> = Some("hey".to_owned());
> assert_eq!(x.as_deref_mut().map(|x| {
>     x.make_ascii_uppercase();
>     x
> }), Some("HEY".to_owned().as_mut_str()));
> ```
>
> ```rust
> //Converts from &mut Option<T> to Option<&mut T>.
> let mut x = Some(2);
> match x.as_mut() {
>     Some(v) => *v = 42,
>     None => {},
> }
> assert_eq!(x, Some(42));
> ```
>
> ```rust
> //Converts from &Option<T> to Option<&T>.
> 
> let text: Option<String> = Some("Hello, world!".to_string());
> // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
> // then consume *that* with `map`, leaving `text` on the stack.
> let text_length: Option<usize> = text.as_ref().map(|s| s.len());
> println!("still can print text: {:?}", text);
> ```
>
> [Rust std::option::Option]: https://doc.rust-lang.org/std/option/enum.Option.html
>
> 注意：对于Option的所有操作，通常以抛出Panic为默认行为的api, 建议用作`原型开发之用`， 或者明确无法继续下去的流程；特别注意，如果此Rust代码通过FFI可以被其他语言，如：C/C++调用时， 请不要抛出panic, 如果一定要抛出， 请用catch_unwind()拦截以免波及其他语言环境，产生未定义行为！而对于Rust调用其他语言代码，如C/C++, 其传递给Rust环境的严重Error, 也可以用resume_unwind()在Rust 环境中抛出panic! 对于C语言本身没有异常和panic的语言元素，但是遇到严重问题，则会触发操作系统信号，如:SIGILL, SIGSEGV等默认action为产生core dump and terminating process；
>
> > The default action for things like `SIGSEGV` is to terminate your process but as you've installed a handler for it, it'll call your handler overriding the default behavior. But the problem is segfaulting instruction may be retried after your handler finishes and if you haven't taken measures to fix the first seg fault, the retried instruction will again fault and it goes on and on.
> >
> > So first spot the instruction that resulted in `SIGSEGV` and try to fix it (you can call something like `backtrace()` in the handler and see for yourself what went wrong)
> >
> > Also, the POSIX standard says that,
> >
> > > The behavior of a process is undefined after it returns normally from a signal-catching function for a [XSI] SIGBUS, SIGFPE, SIGILL, or SIGSEGV signal that was not generated by kill(), [RTS] sigqueue(), or raise().
> >
> > So, the ideal thing to do is to fix your segfault in the first place. **Handler for segfault is not meant to bypass the underlying error condition**
> >
> > So the best suggestion would be- *Don't catch the `SIGSEGV`*. Let it dump core. Analyze the core. Fix the invalid memory reference and there you go!
> >
> > 信号处理原则：不要试图处理所有信号，保持其默认action, 只处理你必须关心的可被catch的signal ;
> >
> > 同时不要试图处理SIGSEGV SIGILL这类signal, 其最好的行为就是core dump and terminating the process。正如这世间，有些事后悔晚矣，只待来生！
> >
> > ---
> >
> > Signal dispositions are process-wide; all threads in a process share the same disposition for each signal. If one thread uses sigaction() to establish a handler for, say, SIGINT, then that handler may be invoked from any thread to which the SIGINT is delivered. 每个子线程继承父线程的signal mask, 同时设定自己的mask 屏蔽不感兴趣的signal, 这样进程范围的signal就只能发给未屏蔽此信号的线程，OS随机挑选；当然也存在发给特定线程的signal.
> >
> > ---
> >
> > **A signal may be directed to either the process as a whole or to a specific thread**. A signal is thread-directed if
> >
> > it is generated as the direct result of the execution of a specific hardware instruction within the context of the thread (**`SIGBUS, SIGFPE, SIGILL, and SIGSEGV`**)
> >
> > >[Reference]
> > >
> > >`http://man7.org/linux/man-pages/man7/signal.7.html`
> > >
> > >`http://www.alexonlinux.com/how-to-handle-sigsegv-but-also-generate-core-dump`
> > >
> > >`https://stackoverflow.com/questions/10202941/segmentation-fault-handling`
> > >
> > >`https://stackoverflow.com/questions/6533373/is-sigsegv-delivered-to-each-thread`



* Result<T,E> 不仅可表达有和无，进一步表达错误的种类

