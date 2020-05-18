#[cfg(os1)]
fn hello() {
    println!("os1 hello");
}

#[cfg(os2)]
fn hello() {
    println!("os2 hello");
}

#[cfg(os3)]
fn hello() {
    println!("o3 hello");
}

#[cfg(feature = "foo_1")]
fn foo_1() {
  println!("foo_1");
}

#[cfg(feature = "foo_2")]
fn foo_2() {
  println!("foo_2");
}

#[cfg(feature = "foo_3")]
fn foo_3() {
  println!("foo_3");
}

fn foo() {
    if cfg!(feature = "foo_1") {
        println!("foo_1");
    } 
    if cfg!(feature = "foo_2") {
        println!("foo_2");
    } 
    if cfg!(feature = "foo_3") {
        println!("foo_3");
    } 
}

fn main() {
    hello();
    //注意宏cfg!(predicate) , 在编译时评估predicate的真假， 即true/false, 然后返回结果。
    let machine_kind = if cfg!(unix) {
        "unix"
      } else if cfg!(windows) {
        "windows"
      } else {
        "unknown"
      };
      
      println!("I'm running on a {} machine!", machine_kind);

      foo();
}
