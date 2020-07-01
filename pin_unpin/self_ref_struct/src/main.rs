
#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe {&*(self.b)}
    }
}

//这个代码例子充分说明了`self-referential structs` 引发的严重bug.
//其实在c/c++中也是一样的， 结构体中一个指针field却指向本结构体中的某个其他field，表面看没什么问题，
//但是一旦此结构体被move到其他内存位置，如下面代码中的swap, 这种按位copy的方式不能将结构体中的指针更新为新内存位置！
//这也就是为什么C++提供移动构造函数等语言特性！而Rust语言默认语义是Move, 而且是按位copy方式， 所以出现上面的问题，
//身在曹营心在汉！其实我觉得C++也只是和稀泥，未能彻底解决！对于变量赋值，对象构造可以用移动构造函数和赋值运算符解决；
//对于底层bit copy/swap这一层面仍然是个漏洞，越过围墙，导致移动构造函数和赋值运算符等语言特性成了稻草人！
//当然C++的语言哲学是：后果自负，作死活该！所以有坑有洞自己看着点！而Rust的语言哲学是：见洞就堵，见坑就填，作死就滚！
//如何优雅高效彻底解决`self-referential structs` ， Rust语言还在反复不断探讨！旷日持久！
//这正式我热爱Rust的地方， 对于每一个语言特性都要千锤百炼，绝不滥竽充数， 而且对于发现潜在问题的语言特性零容忍，坚决砍去！
//所以彻底完整的解决方法还在讨论， 为了实现poll a future,  搞个Pin /UnPin就够用了！
//Pin的意思就是像钉子那样钉在那里不准移动！这样`self-referential structs`的指针问题就没有啦！
//当然对于`非self-referential structs`的类型标记为UnPin , 即使被Pin也无影响，照样Move, 因为不存在自引用，完全安全呀！
fn main() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());
    std::mem::swap(&mut test1, &mut test2);
    test1.a = "I've totally changed now!".to_string();
    println!("a: {}, b: {}", test2.a(), test2.b()); //证明指针test2.b仍然指向test1.a位置，但是他应该指向test2.a才对的！！！

}