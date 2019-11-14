use std::cell::RefCell;
use std::sync::Mutex;
//do ask rust compiler what types are unwindsafe.
fn implements<T: std::panic::UnwindSafe>() {}

fn main() {

    //可变不共享，共享不可变！
   //包括内部可变性！
   //对于可变且共享的元素，可否证明安全？

   //below all is UnwindSafe.
    implements::<Option<i32>>();
    implements::<&Option<i32>>();
    implements::<&Mutex<i32>>();

//below all is not UnwindSafe.
    //implements::<&mut i32>();
    //implements::<&RefCell<i32>>();

}
