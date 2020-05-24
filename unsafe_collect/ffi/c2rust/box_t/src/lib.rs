
#[repr(C)]
#[derive(Debug)]
pub struct Foo;

#[no_mangle]
pub extern "C" fn foo_new() -> Box<Foo> {
    Box::new(Foo)
}

#[no_mangle]
pub extern "C" fn foo_new_option() -> Option<Box<Foo>> {
    Some(Box::new(Foo))
}

// C`s NULL pointer  is equeal to Option::None.
#[no_mangle]
pub extern "C" fn foo_delete(f: Option<Box<Foo>>) {
    println!("{:?}",f );
}