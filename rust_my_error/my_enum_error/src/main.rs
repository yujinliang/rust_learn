mod error;


fn main() {
   
       println!("{}", error::MyError::Test1);
       println!("{}", error::MyError::Test2('c'));
       println!("{}", error::MyError::Test3(8));
       println!("{}", error::MyError::Test4("test errror &str"));
       println!("{}", error::MyError::Test5("test error string".to_string()));

      if let error::MyError::Other(e) =  error::MyError::Other(Box::new(std::io::Error::new(std::io::ErrorKind::Other,  error::MyError::Test1))){
             println!("{}", e);
             assert!(e.is::<std::io::Error>());
             assert!(e.downcast::<std::io::Error>().unwrap().get_ref().unwrap().is::<error::MyError>());
      }
}
