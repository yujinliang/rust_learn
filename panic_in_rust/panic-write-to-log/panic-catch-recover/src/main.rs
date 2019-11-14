use std::panic;

fn main() {


    let result = panic::catch_unwind(|| {
          println!("no panics , all is ok!");
       });
     debug_assert!(result.is_ok());

     let result = panic::catch_unwind(|| {
             panic!("oh panic occured !");
          });
     debug_assert!(result.is_err());

     println!("main thread is ok" );
}
