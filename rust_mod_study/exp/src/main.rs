extern crate lip;
extern crate lip1;
extern crate lip2;
//extern crate lip3; //for rust 2018 edition

mod consumer;
mod producer;
mod switcher;

fn main() {
    println!("exp mod:");

    println!("lip mod");
    lip::caller::call();
    lip::worker::work1();

    println!("lip1 mod");
    lip1::caller::call();
    lip1::worker::work1();

    println!("lip2 mod");
    lip2::caller::callerx::call();
    lip2::worker::worker1::work1();
    lip2::worker::worker2::work2();
    lip2::worker::worker3::work3();
    lip2::worker::workx();

    println!("lip3 mod");
    lip3::caller::callerin::call();

    println!("exp mod");
    consumer::cons::Consumer::call_other_mod();
    producer::Producer::call_other_mod();
}
