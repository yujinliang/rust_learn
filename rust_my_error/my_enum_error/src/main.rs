mod error;

fn main() {
   
       println!("{}", error::MyError::Test1);
       println!("{}", error::MyError::Test2('c'));
       println!("{}", error::MyError::Test3(8));
       println!("{}", error::MyError::Test4("test errror &str"));
       println!("{}", error::MyError::Test5("test error string".to_string()));
       println!("{}", error::MyError::Io(std::io::Error::new(std::io::ErrorKind::Other,  error::MyError::Test1)));
}
