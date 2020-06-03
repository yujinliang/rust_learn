
#[derive(Debug)]
struct SubField(u8);

impl Drop for SubField {
    fn drop(&mut self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct HasDrop
{
    a: SubField,
    b:SubField,
    c:SubField,
    d:SubField,
} //你的自定义类型。

impl Drop for HasDrop {
    fn drop(&mut self) { 
        println!("HasDrop!");
    }
}

fn main() {
    let _x = HasDrop{a:SubField(1), b:SubField(2), c:SubField(3), d:SubField(4)};
}