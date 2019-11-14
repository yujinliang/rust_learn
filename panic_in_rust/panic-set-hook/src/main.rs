use std::panic;

fn main() {

    panic::set_hook(Box::new(|info| {
        println!("Custom panic hook: {:?}", info);
    }));

    panic!("Normal panic");

}
