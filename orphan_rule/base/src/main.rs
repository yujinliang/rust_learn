//base/src/main.rs
//or base/src/lib.rs
//那么base目录就是一个完整独立的crate，里面包括很多子目录。
//每一个crate下，比如base下，都会有一个Cargo.toml用于编译这个crate.


pub trait A {
    fn summarize(&self) -> String;
}
pub struct B {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//注意impl需要满足rust的orphan rule(孤儿规则)， 即A或者B两者至少有一个必须和impl块定义在同一个本地(当前)crate中，否则禁止impl块.
//很明显，当前的A和B都和impl块定义在同一个crate中， 所以合法，也就是说A和B里至少有一个必须是自己亲生的孩子，否则无权调教。
//举个反例， 比如：trait Display和Vec<T>都是rust标准库crate中定义好的，所以你的impl块就违反孤儿规则。
//总的原则 ： 自己的孩子自己管，别人的孩子你管不着。
impl A for B {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


fn main() {
    let b = B{ headline: "headline".to_string(), location: "location".to_string(), author: "author".to_string(), content: "content".to_string()};
    println!("# {}",b.summarize());
}
