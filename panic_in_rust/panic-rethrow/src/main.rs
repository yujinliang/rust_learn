use std::panic;

fn main() {

    let result = panic::catch_unwind(|| {
        panic!("oh no!, panic occured!");
    });

    println!("I am ok 1st", );

    if let Err(err) = result {
        println!("I am ok 2nd", );
        panic::resume_unwind(err);
        //println!("unreachable here", );
    }

    println!("unreachable here", );
}
