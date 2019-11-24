
#[derive(Debug)]
pub enum MyError{

    Test1,
    Test2(char),
    Test3(i32),
    Test4(&'static str),
    Test5(String),
    Io(std::io::Error),
    Other(Box<dyn std::error::Error+Send+Sync>),
}

impl std::convert::From<std::io::Error> for MyError {

    fn from(e: std::io::Error) -> Self {
        MyError::Io(e)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {

            MyError::Test1 => write!(f, "MyError::Test1"),
            MyError::Test2(c) =>  write!(f, "MyError::Test2: {}", c),
            MyError::Test3(n) => write!(f, "MyError::Test3: {}", n),
            MyError::Test4(ref s) => write!(f, "MyError::Test4: {}", s),
            MyError::Test5(ref s) => write!(f, "MyError::Test5: {}", s),
            MyError::Io(e) => e.fmt(f),
            MyError::Other(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for MyError {

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Test1 => None,
            MyError::Test2(..) =>  None,
            MyError::Test3(..) => None,
            MyError::Test4(..) => None,
            MyError::Test5(..) => None,
            MyError::Io(e) => e.source(),
            MyError::Other(e) => e.source(),

        }
    }

}