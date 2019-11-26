mod error;

fn main() {
    
    error::test();

 if  error::ErrorKind::Os == error::get_a_e().kind() {

    println!("os error number: {}", error::get_a_e());
 }
}
