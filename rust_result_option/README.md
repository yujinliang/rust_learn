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



* 对待Option<T> 剥洋葱

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
>     // This will not panic, since all keys are unique.
>     squares.insert(i, i * i).unwrap_none();
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
>     n % 2 == 0
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
>     let maybe_some_string = Some(String::from("Hello, World!"));
>     // `Option::map` takes self *by value*, consuming `maybe_some_string`
>     let maybe_some_len = maybe_some_string.map(|s| s.len());
>     assert_eq!(maybe_some_len, Some(13));
> 
>     //x.map(y_closure) 如果x is None , then return None.
>     let t_none: Option<i32> = None;
>     let none_map = t_none.map(|_x|  3 );
>     assert_eq!(none_map, None);
> 
>     //x.map(y_closure) 如果x is None , then return None.
>     let t_2none: Option<i32> = None;
>     let none_none_map :std::option::Option<std::option::Option<i32>> = t_2none.map(|_x|  None);
>     assert_eq!(none_none_map, None);
>     
>     //x.map(y_closure) 如果x is not None , 则调用闭包y_closure，闭包结果为：None， 最终返回结果
>     //为:Some(None)
>     let closure_r_none: std::option::Option<std::option::Option<i32>>  = Some(6).map(|_x|  None );
>     assert_eq!(closure_r_none, Some(None));
>     
> }
> ```
>
> 

