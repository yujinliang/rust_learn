use std::fmt;

#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Flags: u32 {//模块可见性: private默认本模块可见， pub模块外也可见。
        const A = 0b00000001;  
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

impl Flags {
    pub fn clear(&mut self) {
        self.bits = 0;  // The `bits` field can be accessed from within the
                        // same module where the `bitflags!` macro was invoked.
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hi!")
    }
}

fn main() {
    //example1
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC);   // union
    assert_eq!((e1 & e2), Flags::C);     // intersection
    assert_eq!((e1 - e2), Flags::A);     // set difference
    assert_eq!(!e2, Flags::A);           // set complement
    assert!(e1.contains(Flags::A));
    //example2
    let mut flags = Flags::A | Flags::B;
    flags.clear();
    assert!(flags.is_empty());
    assert_eq!(format!("{}", flags), "hi!");
    assert_eq!(format!("{:?}", Flags::A | Flags::B), "A | B");
    assert_eq!(format!("{:?}", Flags::B), "B");
}