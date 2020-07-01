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
    std::mem::swap(&mut test1, &mut test2); //????????????
    println!("a: {}, b: {}",test2.as_ref().a(), test2.as_ref().b()); //从程序output看test2.as_ref().b()指向正确的位置！
    //我发现pin to stack例子中swap就会编译报错，不允许，或者说禁止获得&mut Test; 
    //但是本例pin to heap 的swap编译通过，运行正确，test1和test2被移动成功， 而且 test2.b指向正确！？
    /*
    Pin<BoxT>> 代表将分配在对上的T类型对象钉在原地不动，从而保持固定的内存位置！切记是T被分配到堆上，并且被钉在原地不许移动！即内存位置固定！ 可并不是钉住Box<>这个胖指针不让Move! 

但是即使不采用Pin , 当对象创建在堆上时， 即使他是!UnPin类型， 同样可以一直保持固定的内存位置！所以分配到堆上的对象天生就有固定性稳定性！！！
此处特别容易令人迷惑， 特别是C++guys对于下面代码可能会疑惑：

 std::mem::swap(&mut test1, &mut test2); //它按位交换的只是两个胖指针自身， 对其所指堆上对象没有影响！

    此行代码命名已经按bitcopy and swap这两个堆对象了，你怎么可以说他们的内存位置不定不变呢？？？？？？？
    
莫急， 听我分说， 首先Rust语言默认就是Move语义， 传递所有权，除非类型实现了Copy Trait； 其二Rust的堆对象都是通过`胖指针`， 如Box来创建的！ ， 形如： let mut boxed = Box::new(t);
boxed只一个栈变量， 一个胖指针， 而其所指的数据对象在堆上！ test1 和test2即是如此， 所以通过swap, =赋值等操作， 只是按位复制交换了栈变量test1和test2, 相当于两者交换了所有权， 交换了指向！ 而堆上的数据对象不受影响！
所以说move的只是这两个胖指针而已！交换了所有权而已， 其所指对象还在堆上内存位置固定没变！ 完全符合Pin<Box<T>>的语义要求。
     */
}