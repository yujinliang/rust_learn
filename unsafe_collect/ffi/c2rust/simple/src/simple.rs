use std::os::raw::c_int;

pub type AddCallback = unsafe extern "C" fn(c_int);

extern "C" {
    pub fn simple_add_two_numbers(a: c_int, b: c_int, cb: AddCallback);
}


pub unsafe extern "C" fn two_numbers_added_cb(result: c_int) {
    println!("Got {}!", result);
}

