# `Hand-Implementing PartialEq, Eq, Hash, PartialOrd and Ord in Rust`

# Introduction

这是一篇简短的指南，指导你实现诸如`相等性、哈希、排序等Rust Trait`, 通常你会采用auto-derive， Rust编译器自动帮我们`impl 某个Trait`, 如下：

```rust
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub MyStruct {
    ...
}
```



但是有时，我们需要可以自己实现，而非auto-derive, 也许自己实现的效率更高， 或者是实现自己认为的相等性和比较关系。

本文我以一个`Rust struct`为例：

```rust
#[derive(Debug, Default, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub contents: String,
    pub is_valid_utf8: bool,
}
```

这个结构体描述了一个文本文件的路径、内容，以及内容是否为`utf8编码`。`file path`具有唯一性， 好比是实体关系中的一个主键， 它是`struct FileInfo`中的决定性成员属性， 通常我们认为两个`file path`相等， 则其对应的`struct FileInfo`也是相等的！



------



# `Equality: the PartialEq and Eq traits`

如果我们想比较某个类型的两值x and y是否相等，或者不等， 如：x == y and x != y， 那么我们必须为类型实现`PartialEq Trait`。



```rust
impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path // 注意此处比较的是path,  FileInfo的一个决定性成员属性。
    }
}
```

此处也可以为`struct FileInfo`实现`Eq Trait` , 通常`PartialEq and Eq`我们一并实现。



注意`Eq Trait`的定义是空的， 没有方法：

```rust
trait Eq: PartialEq<Self> {}
```

然而这并不代表它毫无用处，它是一种`标记性的Trait`, 使类型可用作`hashmaps`中的键。



```rust
impl Eq for FileInfo {} //手动实现一个空impl块。
```

不过还是`#[derive(Eq)]` 更容易和简洁。



------

一个类型什么时候不能实现`Eq`, 非常稀少。`Eq`是一个等价关系， 因此必须满足一定的特性：

- 传递性： if `x == y` and `y == z` then `x == z`
- 对称性： if `x == y` then `y == x`
- 自反性： `x == x` is always true

一个类型中的所有值都必须满足上述特性时才可以实现`Eq`。(一个类型代表一个值域)

这些特性适用于大多数数据类型。主要的例外（也是Rust标准库中唯一的例外）是浮点数值，其中`NaN`的自反性属性不成立，因为IEEE浮点标准要求`NaN`不等于自身（或任何其他数字）。也可以说不可以(不能/不允许)为浮点类型实现`Eq Trait` 。

> **TL;DR** `If you implement PartialEq then #[derive(Eq)] as well unless you can’t`
>
> （`如果你实现了PartialEq , 那么请你也尽可能一并实现Eq, 除非不允许（不能）！`）



------



# Hashing（哈希散列）

散列值与相等的概念密切相关，因此如果实现自己的`PartialEq Trait`，还应该实现`Hash Trait` 。

> The following must hold: if `x == y` then `hash(x) == hash(y)`
>
> 一个原则： if `x == y` then `hash(x) == hash(y)`

如果你的类型不满足上述原则， 那么它就不适合作为一个`HashMap and HashSet`的key。

也就是说，根据`PartialEq `,如果两个值是相等的， 那么他们的哈希值也必然是相等的， 但是反之不成立（不一定成立），因为存在`哈希冲突`， 特别是当一个待哈希的值域远比哈希值自身值域大得多的情况下， 举一个简单的例子：一个`struct` 有两个`u64`的成员， 那么将存在`u64::MAX * u64::MAX`个可能的值实例， 所以不可能将所有`struct`实例都哈希到一个`u64`值域中，必然存在冲突，也可以说不能保证生成`唯一的哈希值`。

`将对FileInfo求哈希值委托给其成员path`:

```rust
impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.path.hash(hasher); //注意此处，hash a FileInfo就是hash其path。
    }
}
```

 这种委托技术适用于所有类型，因为标准库中的所有基本类型都实现了`PartialEq`和`Hash`。



------



# `Ordering（排序）: the PartialOrd and Ord traits（全序和偏序）`

使用运算符`<`、`<=`、`>=`和`>`可以计算值的相对顺序，为此必须为自定义类型实现`PartialOrd`。

> Before you can implement `PartialOrd` you must implement `PartialEq` first.
>
> 特别注意：在你为自定义类型实现`PartialOrd Tait`之前，你必须首先为其实现`PartialEq Trait`。



- 一个例子


```rust
impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}
```



`Ordering` 是一个简单的枚举类型，有3个可能的值：

```rust
pub enum Ordering {
    Less,
    Equal,
    Greater,
}
```

你也许会很好奇，为什么 `partial_cmp` 这个方法返回值类型是个Option, 而非直接是一个`Ordering`值类型？！

这仍然与浮点数类型有关， 因为`NaN`不是一个可以表示的数值， 诸如：`3.0 < NaN`这样的表达式毫无意义！对于这种情况，`partial_cmp`就会返回`None`  因此浮点数是Rust标准库中发生此结果的唯一特例。



`partial_cmp`返回一个`Option`导致一个结果，当结果为`Node`时， 无法决定两个值的排序，即x 和y 会处于不确定排序。实际上， 这意味着只实现`PartialOrd Trait`还不足以使你的自定义类型可排序，你还需要实现`Ord Trait`。

> `To enable your values to be sorted, you must implement **Ord`**
>
> 若要你的自定义类型可排序，你必须为它实现`Ord Trait` 。
>
> `Before you can implement **Ord**, you must first implement **PartialOrd**, **Eq** and **PartialEq**`
>
> 在你实现`Ord Trait` 之前， 你首先必须实现`PartialOrd, Eq, PartialEq Trait`。



```rust
impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path) //将排序委托给其成员path。
    }
}
```

有了它，即上面的`Ord实现`， 就可以排序一个`Vec<FileInfo>`了， 换句话说，这个`Vec<FileInfo>`能排序了！



------



# 自定义类型的多成员参与比较

你可能好奇如何同时比较自定义类型的多个成员。为此如何实现上面提及的多个Trait?(这有点类似于实体关系领域中的复合主键)， 下面的模式（代码例子）正是解决之道：

```rust
impl PartialEq for ExtenededFileInfo {
    fn eq(&self, other: &Self) -> bool {
        // Equal if all key members are equal
        self.path == other.path &&
        self.attributes == other.attributes
    }
}

impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // Ensure we hash all key members.
        self.path.hash(hasher);
        self.attributes.hash(hasher);
    }
}
```

排序是一个繁琐的事情， 你首先比较第一个成员，若是不相等，则一切结束， 若是相等，则你必须接着比较下一个成员， 以此类推，直至完成。剩下的留给读者自己练习吧。



------



# `不同类型的比较`

我有意简单化了上面的讨论， 只是讨论了相同类型值之间的比较，比如两个`FileInfo`值之间的比较。但实际并非如此简单， 上面提及的Trait， 基本都持有一个`Rhs`参数类型， 又称`右值` ， 这样定义非常灵活，它允许你把`FileInfo和Path`作比较， 也允许你把一个复数和`f64`作比较。但是`Ord Trait`例外， 它要求比较的两者必须类型相同（即`self 和 Rhs 必须是相同类型`）。



上面的例子没有讨论`Rhs`, 默认其与Self类型相同， 下面是Rust标准库中`PartialEq`的定义：

```rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}
```



如果想要不同类型间实现比较， 比如`FileInfo and &str`之间的相等性比较， 你可以参考下面的代码：

```rust
impl PartialEq<&str> for FileInfo { //泛型特化
    fn eq(&self, other: &&str) -> bool {
        match self.path.to_str() {
            Some(s) => s == *other,
            None => false
        }
    }
}
```

注意，`eq`中的`other`参数总是被定义为对某个东西的不变引用，因此当我们为`&str` 实现它时，我们最终会得到一个双引用，然后我们必须先解一次引用才能进行`s==*other`比较。



------



# `关于效率`

你也许好奇，到底是自己手动实现的效率高， 还是`auto-derive`效率高？这很难说，不能一概而论，比如：如果你明确定知道一个`大struct`中只有少数几个成员与此比较直接相关，或者说有决定性， 那么你自己手动实现的版本很可能优于`auto-derive`版本， 因为`auto-derive`版本通常会依次比较所有成员，很可能做了无用功。换一个角度说， 尽管`auto-derive`版本可能会依次检查比较每一个`struct`成员， 但是因为它可以采用`布尔表达式的短路原则`， 也许检查第一个成员就停止了，因此也很有可能快于自定义实现版。但是`Hash`是一个例外，它不允许短路原则，必须所有成员依次都要哈希一次才可以，不能偷懒，但如果你能够只哈希1或2个简单成员而不是大量字符串成员，那么您将很容易击败`auto-derive`默认实现。

`auto-derive`有另外两个技巧。首先，它为`Trait`中的**所有**方法生成自定义实现，包括那些具有默认实现的方法。例如，对于`PartialOrd`，它不仅生成`partial cmp`，还生成`lt、le、ge和gt`。其次，它在所有方法中都添加了`#[inline]`。



###### 你可以使用`cargo expand`工具将`#[derive()]`生成的代码打印到`stdout`， 在下面的代码中，你可以自己检验自定义实现和`#[derive()]`生成代码的差异。



- 完整代码：

```rs
use std::path::PathBuf;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Eq)]
pub struct FileInfo {
    pub path: PathBuf,
    pub contents: String,
    pub is_valid_utf8: bool,
}

impl FileInfo {
    fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }
}

impl PartialEq for FileInfo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Hash for FileInfo {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.path.hash(hasher);
    }
}

impl PartialOrd for FileInfo {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl Ord for FileInfo {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialEq<&str> for FileInfo {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        match self.path.to_str() {
            Some(s) => s == *other,
            None => false
        }
    }
}

fn main() {
    // Demonstrate the various traits. Try commenting out the `impl`
    // blocks to see the various compilation errors you get when they
    // are not implemented.

    let f1 = FileInfo::new("/temp/foo");
    let f2 = FileInfo::new("/temp/bar");

    // ------------------------------------------------------------------------------
    // Demonstrate PartialEq. It gives us `==` and `!=`.
    if f1 == f2 {
        println!("f1 and f2 are equal");
    } else {
        println!("f1 and f2 are NOT equal");
    }

    if f1 != f2 {
        println!("f1 and f2 are NOT equal");
    } else {
        println!("f1 and f2 are equal");
    }

    // ------------------------------------------------------------------------------
    // Demonstrate Hash. Note that the HashMap takes ownership of its keys -
    // they are moved into the HashMap.
    let mut hm = HashMap::new();
    hm.insert(f1, 200);
    hm.insert(f2, 500);
    // To avoid complicating this discussion, make a new FileInfo to perform a lookup.
    // In real-life, you would implement the Borrow trait to avoid the temporary variable.
    let f_lookup = FileInfo::new("/temp/foo");
    let file_size = hm[&f_lookup];
    println!("f1 has a size of {} bytes", file_size);

    // ------------------------------------------------------------------------------
    // Demonstrate PartialOrd. It gives us `<`, `<=`, `>=` and `>`.

    // Makes some new f's because the others went into the HashMap.
    let f1 = FileInfo::new("/temp/foo");
    let f2 = FileInfo::new("/temp/bar");

    if f1 < f2 {
        println!("f1 is less than f2");
    } else {
        println!("f1 is not less than f2");
    }

    if f1 > f2 {
        println!("f1 is greater than f2");
    } else {
        println!("f1 is not greater than f2");
    }

    // ------------------------------------------------------------------------------
    // Demonstrate Ord. It unlocks sorting functionality.
    let mut v = vec![f1, f2];
    v.sort();
    println!("v after sorting = {:#?}", v);

    // ------------------------------------------------------------------------------
    // Demonstrate cross-type equality testing.
    let f1 = FileInfo::new("/temp/foo");
    if f1 == "/temp/foo" {
        println!("The path in f1 is equal to the str value \"/temp/foo\"");
    } else {
        println!("Nope, comparisons to strings are not working as they should be.");
    }
}
```

[原文链接:](https://www.philipdaniels.com/blog/2019/vscode-extensions/)

`https://www.philipdaniels.com/blog/2019/rust-equality-and-ordering/`



- 译者

> 学习随笔，水平粗陋，如有谬误，望请海涵雅正，谢谢。
>
> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)
>
> 微信：13718438106