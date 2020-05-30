# 					Too -many-lists学习摘要

- IterMute实现要点难点

```rust
...
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
...
...
...
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

//https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html
//IterMut之所以可以工作， 原文给出了2个理由：
//1. 通过take确实保证了&mut型引用的唯一排他性。
//2. https://doc.rust-lang.org/nomicon/borrow-splitting.html
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        //this works coz: Rust understands that it's ok to shard a mutable reference into the subfields of the pointed-to struct, 
        //because there's no way to "go back up", and they're definitely disjoint.
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {//此处next被take收割了， 保证了map外部无法再访问到！从而确保&mut引用排他性。
            //this works coz: Rust understands that it's ok to shard a mutable reference into the subfields of the pointed-to struct,
            // because there's no way to "go back up", and they're definitely disjoint.
            //We take the Option<&mut> so we have exclusive access to the mutable reference. 
            //No need to worry about someone looking at it again.
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
    ...
    ...
```

> 上面的IterMute代码之所以可以工作，too-many-lists作者给出两点理由：
>
> 1. 通过take确实保证了&mut型引用的唯一排他性。
> 2. Rust知道可以同时借用结构的不相交字段，如下面代码例子。



```rust
//https://doc.rust-lang.org/nomicon/borrow-splitting.html
//https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html

#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let mut x = Foo {a: 0, b: 0, c: 0};
    let xx =&mut x;
    let a = &mut x.a;
    let b = &mut x.b;
    let c = &x.c;
    *b += 1;
    let c2 = &x.c;
    *a += 10;
    println!("{} {} {} {}", a, b, c, c2);
    //println!("whole struct: {:?}", xx);
    //Rust 编译器很聪明，允许你分别持有指向结构体子成员的可变引用，因为Rust编译器知道他们彼此没有交集，是安全的。
    //但是`引用xx`与后续声明的其他引用有交集，很明显struct整体当然包含其子成员！Rust铁律：允许多只读，排他写！所以不安全！
    //但是Rust编译器很聪明，只要`引用xx`无处使用就是安全的！所以注释掉println!就OK, 否则报错：` cannot borrow `x.a` as mutable more than once at a time`

}
```

> too-many-lists中亮点很多，非常值得学习的Rust著作， 而IterMute的实现方法更是创意非凡！不用unsafe也可有这么好的trick! 绝对让人眼前一亮！灵光一闪！值得反复学习。例子代码位置：`https://github.com/yujinliang/rust_learn/tree/master/too_many_lists`  





- Author

> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)
>
> git: https://github.com/yujinliang
>
> `水平有限，万望海涵，指导`





- Reference

`https://doc.rust-lang.org/nomicon/borrow-splitting.html`

`https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html`



