
#[repr(C)]
#[derive(Debug)]
pub struct Foo;

#[no_mangle]
pub extern "C" fn foo_new() -> Box<Foo> {
    Box::new(Foo)
}

// The possibility of NULL is represented with the `Option<_>`.
#[no_mangle]
pub extern "C" fn foo_delete(f: Option<Box<Foo>>) {
    println!("{:?}",f );
}