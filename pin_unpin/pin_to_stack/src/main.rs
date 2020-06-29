//https://cfsamson.github.io/books-futures-explained/4_pin.html#pinning-and-self-referential-structs
use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned, //此struct被标记为: !UnPin , 就是禁止Move的标志。
}


impl Test {
    fn new(txt: &str) -> Self {
        //直接在stack上create Test struct.
        //注意：不论C/C++/Rust等， 栈（变量）对象的生命周期小于等于当前所在函数调用栈帧的生命周期，
        //故此指针指向栈对象， 必须慎重考虑其生命长短，避免出现`悬指针`。
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }
    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.b) }
    }
}

fn main() {
    // test1 is safe to move before we initialize it
    let mut test1 = Test::new("test1");
    // Notice how we shadow `test1` to prevent it from being accessed again
    //同名的新指针变量屏蔽了原来的test1, 以此确保只能通过Pin来访问到Test.
    //这样确保不可能再访问到旧test1指针！
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
    //swap导致编译错误， 因为Pin实质上就是禁止获得&mut T引用(指针) ，
    //无法获得&mut T指针，则无法Move , 比如：swap等。
    //之所以用Pin 包裹原来的裸指针，目的就是禁止获取到：&mut T.
    //好比游子的父母就是不搬离老房子， 这样归来的游子才能找到家门。
   // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
}

/*fn main() {
    let mut test1 = Test::new("test1");
    let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1_pin.as_mut());
    drop(test1_pin); //Pin指针被提前drop , 因为test1未被遮蔽， 后面代码仍然可以访问到， 但是test1已被析构
 
    let mut test2 = Test::new("test2");
    mem::swap(&mut test1, &mut test2);
    println!("Not self referential anymore: {:?}", test1.b); //test1.b == 0x00 ， Pin析构时析构了test1 所指的Test Struct, 其内部指针归0， 
    //所以说不再是自引用。
 }*/