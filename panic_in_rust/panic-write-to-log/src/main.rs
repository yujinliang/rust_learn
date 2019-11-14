use std::panic;
use std::ops::Deref;

fn main() {

    panic::set_hook(Box::new(|panic_info| {

        let (filename, line) = panic_info.location()
                                                                        .map(|loc| (loc.file(), loc.line()))
                                                                        .unwrap_or(("<unknown>", 0));

        let cause = panic_info.payload()
                                                    .downcast_ref::<String>()
                                                    .map(String::deref);

        let cause = cause.unwrap_or_else(|| {

            panic_info.payload()
                                    .downcast_ref::<&str>().map(|s| *s)
                                    .unwrap_or("<cause unknown>")

        });

    println!("Test A panic occurred at {}:{}: {}", filename, line, cause); //write to log here.

}));

    panic!("oh panic!");

}
