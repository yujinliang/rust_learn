use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.b) }
    }
}

pub fn main() {
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    println!("a: {}, b: {}",test1.as_ref().a(), test1.as_ref().b());
    std::mem::swap(&mut test1, &mut test2);
    println!("a: {}, b: {}",test2.as_ref().a(), test2.as_ref().b()); //从程序output看test2.as_ref().b()指向正确的位置！
    //我发现pin to stack例子中swap就会编译报错，不允许，或者说禁止获得&mut Test; 
    //但是本例pin to heap 的swap编译通过，运行正确，test1和test2被移动成功， 而且 test2.b指向正确！？
    //看来Pin坑深呀！慢慢研究吧！
}