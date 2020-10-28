use std::fmt;

//有的时候我们想对别人生的孩子稍加改造，但是rust的orphan rule孤儿规则不允许。
//所以我们必须自己生一个， 虽然他只是个wrapper包装器，可毕竟是自己亲生的，满足孤儿规则。
//比如下面的trait Display和Vec<String>都是rust标准库crate中预定义的， 也就是说别人的孩子我们动不得。
//现在我们自己生一个wrapper类用于包装一下Vec<String> , 然后再应用impl块就合法了。
//也许有人会质疑，我原本打算给Vec<String>添加一个借口（新功能）而已，现在新接口实现了，但是Vec<String>
//原先的接口就不能直接调用了，难道让Wrapper再重新实现吗？
//解决这个问题用不着这么笨的方法， 学学rust里各种智能指针(胖指针)的实现方法就好， 
//只要你的自定义类型impl了trait Deref就搞定了， rust帮你自动解引用。
//详情请看：https://doc.rust-lang.org/book/ch15-02-deref.html

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
 }

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
