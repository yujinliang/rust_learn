use enumflags2::BitFlags;

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
enum Test {
    A = 0b0001,
    B = 0b0010,
    C = 0b0100,
    D = 0b1000,
}

fn main() {

    let a_b = Test::A | Test::B; // BitFlags<Test>
    let a_c = Test::A | Test::C;
    let b_c_d = Test::C | Test::B | Test::D;
    
    // BitFlags<Test>(0b11, [A, B])
    println!("{:?}", a_b);
    
    // BitFlags<Test>(0b1, [A])
    println!("{:?}", a_b & a_c);
    
    // Iterate over the flags like a normal set!
    assert_eq!(a_b.iter().collect::<Vec<_>>(), &[Test::A, Test::B]);
    
    assert!(a_b.contains(Test::A));
    assert!(b_c_d.contains(Test::B | Test::C));
    assert!(!(b_c_d.contains(a_b)));
    
    assert!(a_b.intersects(a_c));
    assert!(!(a_b.intersects(Test::C | Test::D)));
}
