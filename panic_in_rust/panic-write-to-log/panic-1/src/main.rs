use std::thread;
use std::panic;

fn main() {

    let r = thread::spawn(move || {

        panic!("panic occured in child thread.");
        //println!("child is Ok", ); //unreachable statement

    }).join();

    println!("{:?}",r );
    println!("althought some panic occurred in child, but the panic  cannot sent back to and break the main thread.");

//------
println!("----------------------------------------------------------------" );
let r = thread::spawn(move || {

    let r = thread::spawn(move || {

        panic!("thead2 panic!");
        //println!("thread2 is Ok", ); //unreachable statement

    }).join();
    println!("the  thread1  is ok" );
    return r;

}).join();

println!("{:?}",r );
println!("main thread is ok");

}
