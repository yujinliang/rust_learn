use async_std::task;
use std::pin::Pin;

struct Circle {
    radius:f64
}
impl Circle {
    async fn area(self : Pin<Box<Self>>) -> f64 {
        let join_handle = task::spawn(async move {
            2.0*3.1415*self.radius  //spawn need : self is required to live as long as `'static` here
        });
        join_handle.await
    }

}
fn main() {
    task::block_on(async {
        let by_arc = Box::pin(Circle{radius:30.5});
        println!("{}", by_arc.area().await);
    })
}