# RUST模块的理解

***



* 每一个crate就是一个根模块。如：`exp, lip, lip1, lip2, lip3`。

* 独立的一个文件就是一个mod,文件名就是mod名；但是main.rs, lib.rs, mod.rs除外，mod.rs的模块名就是其所在目录的名字； 而main.rs, lib.rs 的目录结构，如：` exp/src/main.rs `或 `lip/src/lib.rs` ;两者的mod名分别是exp和lip。

* 文件和文件夹内的mod 及其内部定义的函数默认都是private的，除非pub声明公开。

* 一个文件夹直接包含mod.rs ，如: ` rust_mod_study/lip2/src/worker/mod.rs ;`则 worker就是模块名； 并且mod.rs为此模块的入口文件，此文件夹内的其他子模块都要在mod.rs中 `pub mod 模块名`，声明后，外部方可看到。

* 如果一个元素是私有的，那么只有本模块内的元素以及它的子模块可以访问。

* 如果一个元素是公开的，那么它上一层的模块就有权访问它。

* 如果存在与文件同名的目录， 则在该目录下定义的模块都是该文件的子模块.（`2018 edition有效`）如：`rust_mod_study/lip3/src/caller.rs ;` `rust_mod_study/lip3/src/caller/callerin.rs;`

​       特别注意，callerin这个mod必须在caller.rs中以`pub mod callerin;`形式声明，否则外部看不到； 最终模块路径为：`lip3::caller::callerin::call();`

* `rust 2018 edition` 不再需要在根模块中使用extern crate xxx;语法导入第三方包。如在文件`main.rs , lib.rs`中不再需要extern crate xxx语法导入第三方包, 如:` rust_mod_study/exp/src/main.rs` 中的extern crate xxx可以去掉了。只需在Cargo.toml中配置好， 然后在代码中以模块路径访问即可，如：modx::mody::modz::fnx()； 也可以use一下，缩短路径。

* rust 如何引用未发布的本地crate, 特别之处在`exp/Cargo.toml`中， 如：

```tom
[package]
name = "exp"
version = "0.1.0"
authors = ["yujinliang <285779289@qq.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
lip = {path= "../lip" }
lip1 = {path= "../lip1" }
lip2 = {path= "../lip2" }
lip3 = {path= "../lip3" } 

```

- 目录结构：` Cargo.lock  Cargo.toml  exp  lip  lip1  lip2  lip3  target  `在同一个父目录`rust_mod_study`中；其中`exp/src/main.rs`引用lip开头的所有模块。



- 配置`rust workspace`, 在`rust_mod_study/Cargo.toml`中加入以下配置即可，如：

         ```
         [workspace]
         members = ["exp", "lip", "lip1", "lip2", "lip3"] 
         ```

* 在rust_mod_study/exp中给出一个例子用于说明：在同一个crate下各个子mod间的可见性和引用方法

  1. 首先各个子mod都需要在main.rs(属于crate顶级mod)中声明自己， 如： mod producer; mod consumer; mod switcher;等 ，只有这样各个子mod才能看到彼此，才能引用。
  2. 每一个子mod可以用use crate::xxx形式引用在1.中声明的mod, 如：use crate::producer;等。
  3. 每一个子mod自身默认都是自私的，除非以pub , pub use等打开为公用。
  4. 对于pub struct 其field默认仍然是private的，需要pub声明为公用。

  总结： 父mod可以引用其子mod, 但是在父模块中仍然需要声明后方可应用子模块，如：`mod 子模块名` ；而每一个子模块，只可以看到在其父模块中声明过的子mod,  但是仍需`use crate::子模块名` 来引用一下后方可用。

  我是在rust 1.39中做的实验， 时间急促，水平有限，如有谬误，欢迎指正，感谢啦！

  

* 所有的例子代码都在rust_mod_study目录中 `



[^footnote]: 本文只是随笔，所以不求面面俱到，只针对容易误解的点。



> RUST学习随笔，如有谬误，尽请指正，谢谢。
>
> 作者：心尘了
>
> email: 285779289@qq.com
>
> 微信：13718438106
>



#### Reference List

* 深入浅出Rust, 范长春著， 机械工业出版社.
* Rust编程之道， 张汉东著， 电子工业出版社.



#### Thanks All

####  信息资料繁多，网络信息资源更是浩如烟海，故此难以全部列出引用参考出处，在此一并感谢！如有缺漏，十分抱歉，最后感谢所有前辈的付出和心血。



