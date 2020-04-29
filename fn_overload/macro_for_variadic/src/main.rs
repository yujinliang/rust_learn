macro_rules! sum {
    ($($args:expr),*) => {{
        let result = 0;
        $(
            let result = result + $args;
        )*
        result
    }}
}

macro_rules! print_all {
    ($($args:expr),*) => {{
        $(
            println!("{}", $args);
        )*
    }}
}

fn main() {
    assert_eq!(sum!(1, 2, 3), 6);
    print_all!(1, 2, "Hello");
}

