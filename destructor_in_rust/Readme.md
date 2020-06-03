# 					Rust 析构函数

- #### Trait [std](https://doc.rust-lang.org/std/index.html)::[ops](https://doc.rust-lang.org/std/ops/index.html)::Drop

  如果你的`自定义类型`需要自己的析构函数，则只要实现`std::ops::Drop trait` 即可。例如：

  

```rust
struct HasDrop; //你的自定义类型。

impl Drop for HasDrop {
    fn drop(&mut self) { //std::ops::Drop trait 只有一个方法drop。
        println!("Dropping!"); //当你的自定义类型变量离开了`作用域`时， 析构函数drop被自动调用！
        //切记不允许手动直接调用。 
        //在此处释放各种资源。
    }
}

fn main() {
    let _x = HasDrop; 
    //如果你想提前析构释放变量，缩短变量作用域，可以手动直接调用`std::mem::drop`
    //drop(_x);
}

```



------



```rust
struct Droppable {
    name: &'static str,
}

//注意对于有子成员的自定义类型，如上面的结构体`Droppable` , 析构函数调用顺序为： 先整体后局部， 先父后子！
//即：Droppable::drop --> str::drop , 即依次调用每个子成员的析构函数。

// 为你的自定义类型实现std::ops::Drop trait 
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
        //此处释放各种资源
    }
}

fn main() {
    let _a = Droppable { name: "a" };
    
    //每一对儿花括号就是一个作用域！
    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // 调用`std::mem::drop`可以手动提前释放。
    drop(_a);
   

    println!("end of the main function");

    // `_a` 变量原本应该在离开最后一个花括号时，开始析构。
    // 但是因为前面已经提前手动调用`std::mem::drop`释放了变量， 所以此处不再重复析构。
}

```

【结构体及其子成员析构顺序验证例子】

```rust
#[derive(Debug)]
struct SubField(u8);

impl Drop for SubField {
    fn drop(&mut self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct HasDrop
{
    a: SubField,
    b:SubField,
    c:SubField,
    d:SubField,
} //你的自定义类型。

impl Drop for HasDrop {
    fn drop(&mut self) { 
        println!("HasDrop!");
    }
}

fn main() {
    let _x = HasDrop{a:SubField(1), b:SubField(2), c:SubField(3), d:SubField(4)};
}

/*
* Program Output:
*HasDrop!
*SubField(1)
*SubField(2)
*SubField(3)
*SubField(4)
*/
//从程序输出确认析构顺序： 先父后子， 而每一个子成员析构顺序由其定义顺序决定。
```



- 官方文档-析构顺序规定

> The destructor of a type consists of
>
> 1. Calling its [`std::ops::Drop::drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) method, if it has one.
> 2. Recursively running the destructor of all of its fields.
>    - The fields of a [struct](https://doc.rust-lang.org/reference/types/struct.html), [tuple](https://doc.rust-lang.org/reference/types/tuple.html) or [enum variant](https://doc.rust-lang.org/reference/types/enum.html) are dropped in declaration order. *
>    - The elements of an [array](https://doc.rust-lang.org/reference/types/array.html) or owned [slice](https://doc.rust-lang.org/reference/types/array.html) are dropped from the first element to the last. *
>    - The captured values of a [closure](https://doc.rust-lang.org/reference/types/closure.html) are dropped in an unspecified order.
>    - [Trait objects](https://doc.rust-lang.org/reference/types/trait-object.html) run the destructor of the underlying type.
>    - Other types don't result in any further drops.
>
> \* This order was stabilized in [RFC 1857](https://github.com/rust-lang/rfcs/blob/master/text/1857-stabilize-drop-order.md).
>
> Variables are dropped in reverse order of declaration. Variables declared in the same pattern drop in an unspecified ordered.
>
> 详情请看：`https://doc.rust-lang.org/reference/destructors.html` 和`https://github.com/rust-lang/rfcs/blob/master/text/1857-stabilize-drop-order.md`



- #### [RAII](https://doc.rust-lang.org/rust-by-example/scope/raii.html#raii)

在C++中很有名的一种资源获取释放模式！主要利用类型的析构函数来及时地自动地清理释放持有的资源，避免资源泄露， 不啰嗦了， 详情请看：`https://doc.rust-lang.org/rust-by-example/scope/raii.html`



- 一个析构函数编译报错的例子--所有权争夺

> error: `cannot move out of type `SqlTransaction<'a>`, which defines the `Drop` trait [E0509]at line 12 col 22`

```rust
struct SqlTransaction<'a> {
    connection: &'a Connection,
    transaction: Transaction<'a>,
}

impl<'a> Drop for SqlTransaction<'a> {
    fn drop(&mut self) {
        let result = self.transaction.commit(); //此行引发编译报错，因为commit(self)会转移走`transaction`所有权。
        //双方争夺`transaction`所有权， 可以将`transaction`包裹到Option中， 然后以Option::take让self放弃`transaction`的所有权； 或者替换掉commit(self)，换成引用类的接口。详情请看：`https://stackoverflow.com/questions/34278607/can-not-move-out-of-type-which-defines-the-drop-trait-e0509`
        match result {
            Ok(_) => print!("herp"),
            Error => print!("lol"),

        }
    }
}
```





- Author

> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)
>
> git：https://github.com/yujinliang
>
> `水平有限，笔记草乱，难免谬误，海涵！`



- Reference

`https://doc.rust-lang.org/rust-by-example/trait/drop.html`

`https://doc.rust-lang.org/std/ops/trait.Drop.html`

`https://doc.rust-lang.org/std/mem/fn.drop.html`

`https://stackoverflow.com/questions/34278607/can-not-move-out-of-type-which-defines-the-drop-trait-e0509`

`https://doc.rust-lang.org/nomicon/destructors.html`

`https://doc.rust-lang.org/reference/destructors.html`

`https://github.com/rust-lang/rfcs/blob/master/text/1857-stabilize-drop-order.md`

`https://doc.rust-lang.org/nomicon/constructors.html`

`https://doc.rust-lang.org/rust-by-example/scope/raii.html`