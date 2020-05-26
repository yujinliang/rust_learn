# Rust Iterator迭代器-学习随笔

我认为熟记吃透Rust Option、Result、Iterator这3块的方法集，非常有助于写出简洁高效符合Rust Style的代码！原理性的东西好多前辈都讲过了，我就不啰嗦了！这三块的方法功用必须要记牢！我收集了几个常用方法的小例子，方便查询记忆而已。

> * iter()  => &T
> * iter_mut() => &mut T
> * into_iter() => T



- 实现迭代器

​        `详情请参考：https://doc.rust-lang.org/std/iter/trait.Iterator.html`

​		***只要实现Iterator Trait即可被遍历，官方文档给出的小例子，如下：***

```rust
// First, the struct:

/// An iterator which counts from one to five
struct Counter {
    count: usize,
}

// we want our count to start at one, so let's add a new() method to help.
// This isn't strictly necessary, but is convenient. Note that we start
// `count` at zero, we'll see why in `next()`'s implementation below.
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Then, we implement `Iterator` for our `Counter`:

impl Iterator for Counter {
    // we will be counting with usize
    type Item = usize;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        self.count += 1;

        // Check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// And now we can use it!

let mut counter = Counter::new();

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);

let x = counter.next().unwrap();
println!("{}", x);
```



- Trait IntoIterator for `for loop`


`详情请参考：https://doc.rust-lang.org/std/iter/trait.IntoIterator.html`

标准库容器，包括自定义类型，如果可以被for遍历，形如 for x in v ；for x in &v; for x in &mut v; 需要实现Trait IntoIterator，对于容器v不同的引用方式，直接决定最终生成的Iterator对于容器中元素的指向关系！

实现Trait IntoIterator, 只要调用其into_iter()方法就可以生成相应的迭代器Iterator ，有了容器的Iterator之后，for就可以遍历容器了！注意IntoIterator的名字很直白，就是变成一个对应的Iterator, 分为3种，如：T, &T, &mut T ，即对于容器中元素区分3种指向关系：拥有所有权、借用、可读可修改；

不限于for , 只要一个类型实现Trait IntoIterator, 那么调用into_iter()方法就可以获得相应的Iterator ! 形如：a_var.into_iter()  、 （&a_var).into_iter()、 (&mut a_var).into_iter() ， 从而生成相应的Iterator： 拥有所有权、只读引用、可读写引用。

【into_iter注意事项】

```rust
fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter(); //一旦调用into_iter, 则v的所有权被转移进iter, 后面语句若再访问v, 则编译报错。
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(None, iter.next());
    //println!("{:#?}", v); //compilation error: ^ value borrowed here after move，v已经失去所有权，故此编译报错。
}
```

> 强调一下: move语义是所有Rust数据类型默认语义，而只有实现了std::marker::Copy trait 的类型，才会执行复制语义。 如基本数据类型：数字、字符、bool等都实现了Copy trait。

------









* Author

> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)
>
> 微信：13718438106
>
> ​	`水平有限，笔记草乱，如有谬误，尽请指正！`



- Reference


> `《深入浅出Rust》 ，范长春著，机械工业出版社`
>
> `https://danielkeep.github.io/itercheat_baked.html`
>
> `https://github.com/rustomax/rust-iterators`
>
> `https://www.worthe-it.co.za/programming/2019/08/01/rust-iterators-cheatsheet.html`
>
> `https://doc.rust-lang.org/std/iter/trait.IntoIterator.html`
>
> `https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct`
>
> `https://rust-unofficial.github.io/too-many-lists/second-into-iter.html`
>
> 

   

   

   







