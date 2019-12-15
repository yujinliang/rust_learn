# **Rust 协变，逆变，不变**

> 对于逆变和协变的判断，有时候观察角度不同，结论可能不同，不必纠结，明白原理就好。这个笔记是我学习随笔，不严谨，也没有多方考证，只是记录一下自己的理解，以备他日进一步考证！



> 在rust中， 'a : 'b  表达'a为'b的子类，'b为父类，子类生命周期大于等于父类。一般面向对象编程中涉及父类、子类、型变等概念， 用于表达父类与子类之间的相互替代关系。
>
> 型变Variance
>
> - 不变 `Invariance`
>
>   如果不能将一个类型替换为另一个类型，那么这个类型就称之为：不变。
>
> - 逆变 `contravariant`
>
>   可以由其基类替换。
>
> - 协变 `covariance`
>
>   可以由其派生类型替换。



> 在面向对象中，有个里氏替换原则，大意就是说：任何基类可以出现的地方，子类一定可以出现。
>
> Rust生命周期协变： 扩大生命周期，短命可以指向长命， 父类可以被子类替代。
>
> Rust生命周期逆变：缩小生命周期， 可以接受比声明更命短的参数， 子类可以被父类代替， 作用域更短， 生命更短。



> Rust官方文档关于Variance给出了一个表：

|      |                 | 'a        | T               | U         |
| ---- | --------------- | --------- | --------------- | --------- |
| *    | &'a T           | covariant | covariant       |           |
| *    | `&'a mut T`     | covariant | invariant       |           |
| *    | Box<T>          |           | covariant       |           |
|      | `Vec<T>`        |           | covariant       |           |
| *    | `UnsafeCell<T>` |           | invariant       |           |
|      | Cell<T>         |           | invariant       |           |
| *    | `fn(T) -> U`    |           | `contravariant` | covariant |
|      | `*const T`      |           | covariant       |           |
|      | `*mut T`        |           | invariant       |           |

上表为Rust官网提供，据说可能有变化调整，到时再说，现在主要理解原理，我的理解：

>  &'a T ， 对于'a是协变， 对于T也是协变。对于'a协变，可以理解为：当然可以指向比`a所代表的生命周期更长的元素，这个更长的生命周期以'b代表，
>
> 在rust中程'b 为'a的子类， 即: 'b: 'a ；故此通俗地说： 子代父。因为只读T， 宽松一点，父子皆可。好比村里开会传达一下上面的精神，老爹不再， 儿子当然可以代替， 不会产生大问题。即此类型引用即可以指向父类，也可以指向子类。
>
> //--------------
>
> `&'a mut T` 对于'a协变，解释同上。 对于T不变。对于'a的解释同上； 对于T可写，则含糊不得， 好比村里要为村民办医保， 需要村民自己签字和拍照， 则老爹代替不了儿子， 儿子也代替不了老爹。所以对T不变， 就是限制老爹替儿子或儿子替老爹。
>
> //---------------
>
> `Box<T> 和Vec<T> `对于T都是协变， 因为只读不可变， 所以去村里开会传达精神，儿子老子谁去都可以。
>
> //--------------
>
> `UnsafeCell<T> 和Cell<T> `对于T都是不变， 因为其内部可变性， 村里开会不只是传达精神， 还要签字照相， 故此含糊不得。父子不得相互替代。
>
> //--------------
>
> `fn(T) -> U` 对于T是逆变， 对于U是协变， 对于U协变很好理解，返回比U命更长的子类当然可以啦；即函数返回值可以更加特化，子类化。对于T逆变， 我的理解：可以接受比声明更命短的参数，  子类可以被父类替代。
>
> //-------------
>
> `*const T` 对T协变， 道理很显然， 因为只读， 父子当然可以代替，也就是说这个指针类型即可以指向父类，也可以指向子类。
>
> //-----------------
>
> `*mut T `对T为不变， 道理也很显然， 可写代表村里开会需要自己亲自签字拍照，含糊不得， 父子也不能相替代。
>
> //-----------------
>
> 下面为Rust官方文档给出的总结：
>
> - `Vec` and all other owning pointers and collections follow the same logic as Box
> - Cell and all other interior mutability types follow the same logic as `UnsafeCell`
> - `*const` follows the logic of `&T`
> - `*mut` follows the logic of `&mut T` (or `UnsafeCell`)



> 关于`Box<T>和Vec<T>`对T为协变的深入理解
>
> 从生命周期角度看，当然合理，因为子类可以替代父类，意味着指向的变量生命周期更长，当然没问题，不会出现dangle pointer。在Rust， 协变，逆变和不变等形变概念只是应用到`生命周期`描述中，或者说只针对生命周期起作用。不要理解为声明为`Vec<T>,`但是却push进T的子类对象，那样会爆炸的！更何况Rust Structure等类型也不支持继承，只有Trait可以继承，但非是面向对象编程所表达的父类子类继承。
>
> 啰嗦折磨多，对于`Box<T>或Vec<T>`之类中的T, 可以理解为被生命周期参数修饰的类型的简写，父类子类只针对生命周期参数而言。



> 按照教课书和Rust官网文档给出的数学公式方式的定义，我很难理解协变和逆变对于Rust生命周期的意义，所以大白话一些，自己好记，也许不对，将来再说！先写下来，以备将来考证。其实整的这么复杂，归齐一条，就是搞清楚各个变量，谁的命长，谁的命短，别出现dangle pointer!  短命的可以指向长命的，没问题，但反过来就很可能出现dangle pointer。对于栈变量其生命的长短就是其作用域的大小。



`都是自己的理解，难免谬误，尽请指教，谢谢`

* Reference

1. `https://www.jianshu.com/p/0d60c148c0c0`
2. `https://doc.rust-lang.org/nomicon/subtyping.html#variance`
3. 深入浅出Rust, 范长春著， 机械工业出版社
4. Rust编程之道，张汉东著， 电子工业出版社
5. `百度百科`


