use cpp::*;

cpp!{{
    #include <stdio.h>
    }}
    
    fn add(a: i32, b: i32) -> i32 {
        unsafe {
            cpp!([a as "int32_t", b as "int32_t"] -> i32 as "int32_t" {
                printf("adding %d and %d\n", a, b);
                return a + b;
             })
        }
    }
    
fn main() {
    println!("{}", add(1, 7));
}
