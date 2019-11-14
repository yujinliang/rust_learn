use std::thread;
use std::panic;
use std::time;
use std::any::Any;

fn main() {

        println!("Entering main!");

        let h = thread::spawn(|| {

            let dur_millis = time::Duration::from_millis(500);
            thread::sleep(dur_millis);
            panic!("boom");

        });

        let r = h.join();
        handle(r);

        let r = panic::catch_unwind(|| {

            let dur_millis = time::Duration::from_millis(500);
            thread::sleep(dur_millis);
            panic!(String::from("boom again!"));

        });

        handle(r);

        println!("Exiting main!");

    }

//both of the panic::catch_unwind and thread::spawn  is to return Err(Any)
//fn handle(r: thread::Result<()>) {
fn handle<T: std::fmt::Debug>( r:  Result<T, Box<dyn Any + Send + 'static>>) {

        println!("{:?}", r );
        match r {

            Ok(r) => println!("All is well! {:?}", r),
            Err(e) => {
                if let Some(e) = e.downcast_ref::<&'static str>() {
                    println!("Got an error: {}", e);
                }
                else if let Some(e) = e.downcast_ref::<String>() {

                        println!("Got an error: {}", e);
                }
                 else {
                    println!("Got an unknown error: {:?}", e);
                }
            }
        }

}
