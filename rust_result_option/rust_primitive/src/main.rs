use std::fs::File;
use std::io::ErrorKind;

//pub fn open<P: AsRef<Path>>(path: P) -> Result<File>

fn main() {

    let f = File::open("hello.txt");

    let _f = match f {

        Ok(file) => file,
        Err(error) => match error.kind() {

            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };
}
