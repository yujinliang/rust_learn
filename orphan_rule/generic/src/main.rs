/*https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html
 *rust orphan rule孤儿规则对于泛型做出了适当的放松限制，形如：
 *impl<T> ForeignTrait<LocalType> for ForeignType<T> {
 *   // ...
 * }
 *这样的形式在rust1.41之前是非法的，不满足孤儿规则，虽然ForeignTrait<LocalType>中，
 *以一个本地crate自定义LocalType实例化后，就相当于变成了一个本地localTrait了， 
 *但是旧版本的孤儿规则不允许，分明就是误伤，不过rust1.41之后就允许了。
 *这样极大的增强了泛型表达和扩展能力！
 *https://doc.rust-lang.org/std/convert/trait.Into.html#implementing-into-for-conversions-to-external-types-in-old-versions-of-rust
 *https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html
*/

struct Wrapper<T>(Vec<T>);


impl<T> From<Wrapper<T>> for Vec<T> {

    fn from(w: Wrapper<T>) -> Vec<T> {
        w.0
    }
}


fn main() {
    println!("Hello, world!");
}
