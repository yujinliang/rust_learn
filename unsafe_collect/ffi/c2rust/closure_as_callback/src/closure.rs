use std::os::raw::{c_int, c_void};

pub type AddCallback = unsafe extern "C" fn(c_int, *mut c_void);

extern "C" {
    fn better_add_two_numbers(
        a: c_int,
        b: c_int,
        cb: AddCallback,
        user_data: *mut c_void,
    );
}

pub fn add_two_numbers<F>(a: i32, b: i32, on_result_calculated: F)
where
    F: FnMut(i32),
{
    unsafe {
        let mut closure = on_result_calculated;
        let cb = get_trampoline(&closure);

        better_add_two_numbers(a, b, cb, &mut closure as *mut _ as *mut c_void);
    }
}
//C语言只认识函数指针， 而Rust语言的闭包只是个语法糖，即一个匿名结构体实现了Fn/FnMut/FnOnce，从而实现函数调用。
// trampoline中文蹦床的意思，很是贴切！首先从Rust传递回调函数指针给C， C代码运行相应逻辑后调用回调函数， 从C返回Rust!
//核心思想： 从C回到Rust语言环境， Rust语言当然认识自己的闭包啦！所以可以直接调用。
//注意： 闭包被当做user_data来回传递， 且闭包的声明与FnMut(c_int)必须相同。
//所以FFI的核心原则：求同存异！各找各妈！
unsafe extern "C"  fn trampoline<F>(result: c_int, user_data: *mut c_void)
where
    F: FnMut(c_int),
{
    let user_data = &mut *(user_data as *mut F); //此处并未对user_data做类型检查，直接转换为F, 若是类型不匹配则panic。
    user_data(result); //所以要求闭包声明必须和F一致。
}

//下面这个getter的使命就是帮助Rust编译器实例化fn trampoline。
//因为fn trampoline自己的函数参数都没有用到F泛型参数， 故此Rust没有足够的信息推导出F的具体类型，
//所以下面的getter提供了F的具体类型信息。
fn get_trampoline<F>(_closure: &F) -> AddCallback
where
    F: FnMut(c_int),
{
    trampoline::<F>
}