use std::convert::From;
use std::fmt::{Debug, Display,Formatter};

//只是对std::io::Error实现的简化，以利说明。
#[derive(Debug)]
pub struct MyStructError {
    repr: Repr,
}

impl MyStructError {

    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Os(..) => ErrorKind::Os, //此处应获取OS的enum error kind, 如:  Repr::Os(code) => sys::decode_error_kind(code),
            Repr::Custom(ref c) => c.kind,
            Repr::Simple(kind) => kind,
        }
    }
}

#[derive(Debug)]
enum Repr {
    Os(i32), //用于包含底层OS返回的错误码.
    Simple(ErrorKind), //用于包含自己定义的具体错误case.
    Custom(Box<Custom>), //用于包含其他crate实现的各种Error。
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind, //用于标定其他crate的Error
    error: Box<dyn std::error::Error+Send+Sync>, //用于指向其他crate实现的Error类型。
}

//定义错误具体种类。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {

    NotFound,
    Os,
    Other,
}

impl ErrorKind {
    
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::NotFound => "entity not found",
            ErrorKind::Os => "os error",
            ErrorKind::Other => "other os error",
        }
    }
}

impl From<ErrorKind> for MyStructError {
    #[inline]
    fn from(kind: ErrorKind) -> MyStructError {
        MyStructError {
            repr: Repr::Simple(kind)
        }
    }
}

//此处只是举一个其他crate的Error向我们自定义错误转化的例子。
//other crate `s Error to MyStructError conversion
impl From<std::io::Error> for MyStructError {
    #[inline]
    fn from(o: std::io::Error) -> MyStructError {
        MyStructError {
            repr: Repr::Custom(Box::new(Custom{kind:  ErrorKind::Other, error:Box::new(o)})),
        }
    }
}

impl Display for MyStructError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.repr {
            Repr::Os(code) => {write!(f, "os error code : {}",  code)}
            Repr::Custom(ref c) => Debug::fmt(&c, f),
            Repr::Simple(kind) => write!(f, "{}", kind.as_str()),
        }
    }
}

//自定义错误，不管是enum,还是struct实现，最终都要统一继承实现std::error::Error,这样rust 所有的Error都有一个父亲，
//好处是一个父类型指针就可以指向所有错误类型实现，方便统一管理.
impl std::error::Error for MyStructError {

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.repr {
            Repr::Os(..) => None,
            Repr::Simple(..) => None,
            Repr::Custom(ref c) => c.error.source(),
        }
    }

}

pub fn test() {

    let e1 = MyStructError{repr: Repr::Os(-1)};
    println!("{}",e1);
    println!("{}", MyStructError{repr: Repr::Simple(ErrorKind::NotFound)});

    let other = std::io::Error::new(std::io::ErrorKind::Other,  e1);
    let e2 = MyStructError { repr: Repr::Custom(Box::new(Custom{kind:  ErrorKind::Other, error:Box::new(other)})),};   
    println!("{}", e2);

}

pub fn get_a_e () -> MyStructError {

    MyStructError{repr: Repr::Os(-1)}
}