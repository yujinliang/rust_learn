#[derive(Debug)]
struct Foo {
    value:u64
  }
  
  trait HasUIntValue {
    fn as_u64(self) -> u64;
  }
  
  impl Foo {
    fn add<T:HasUIntValue>(&mut self, value:T) {
      self.value += value.as_u64();
    }
  }
  
  impl HasUIntValue for i64 {
    fn as_u64(self) -> u64 {
      return self as u64;
    }
  }
  
  impl HasUIntValue for f64 {
    fn as_u64(self) -> u64 {
      return self as u64;
    }
  }
  
  fn test_add_with_int()
  {
    let mut x = Foo { value: 10 };
    x.add(10i64);
    assert!(x.value == 20);
    println!("{:?}", x);
  }
  
    fn test_add_with_float()
  {
    let mut x = Foo { value: 10 };
    x.add(10.0f64);
    assert!(x.value == 20);
    println!("{:?}", x);
  }


fn main() {
    test_add_with_int();
    test_add_with_float();
}
