#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod caller {

   pub fn call(){
        
        println!("lip::caller::call()");
    }

}

pub mod worker {

  pub  fn work1() {
    
        println!("lip::worker::work1()");

    }

}
