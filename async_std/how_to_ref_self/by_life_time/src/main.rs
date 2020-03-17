use async_std::task;
//use async_std::task::JoinHandle;
use futures::{future, Future,FutureExt};
use std::error::Error;

struct Circle {
    radius:f64
}

impl Circle {
   async  fn make_a_future<'a>(&'a self) -> impl Future<Output=f64> +'a {
        //below code compilation failed, coz ref self, unless add move for async block.
        async  move {
            2.0*3.1415*self.radius 
        }
    }

    async  fn make_b_future<'a>(&'a self) -> impl Future<Output=f64> +'a {
        future::ok(self).map(|pa:Result<&'a Self, Box<dyn Error+Send>>| {2.0*3.1415*pa.unwrap().radius})
     }

}
fn main() {
    task::block_on(async {
        let by_life = Circle{radius:30.5};
        //1.for task::spawn
        //compilation failed with task::spawn.
       //let r = task::spawn(by_life.make_a_future()).await;

       //2.for async block
       let ra = by_life.make_a_future().await;
        println!("{:?}", ra.await);
        
      //3.for future
        let rb = by_life.make_b_future().await;
        println!("{:?}", rb.await);
    })
}
