extern crate thiserror_t;
use anyhow::{Context, Result};
use std::fs::File;
use std::env;
use thiserror_t::count_words;

fn main() -> Result<()> {
    for filename in env::args().skip(1).collect::<Vec<String>>() {
        let mut reader = File::open(&filename).context(format!("unable to open '{}'", filename))?;
        let wordcount =
            count_words(&mut reader).context(format!("unable to count words in '{}'", filename))?;
        println!("{} {}", wordcount, filename);
    }
    Ok(())
}

//深入学习请看：`https://nick.groenen.me/posts/rust-error-handling/`
//anyhow crate的好处：
//1. 明显简化了main()函数的返回值类型声明。
//2. 通过context可以对相应错误进行更加详细的注释描述。
//3. 通过此命令行：RUST_BACKTRACE=1 cargo run some_options 列出错误源头和传播路径。
//4. 可格式化错误输出显示格式。
//总结： Rust的错误处理方案始终是争论比较激烈的地方， 一直在激烈快速进化，结合几十年丰厚的软件工程实践经验，
//不断探索改进最优雅高效的Rust错误处理方案！最终尘埃落定之前，必然百家争鸣百花齐放！
//所以failure之类已经过时， 现在推荐thiserror 和anyhow crates。
//从示例代码可知， 使用thiserror和anyhow crate极为简单，无痛零负担，好似百步神拳无影掌。
//之前的failure crate有点喧宾夺主的感觉，另立一套，融入性差，使用负担重。