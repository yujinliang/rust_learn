use async_std::task;
use async_std::sync::Arc;

struct Circle {
    radius:f64
}
impl Circle {
    async fn area(self:Arc<Self>) -> f64 {
        let self_divide = self.clone();
        let join_handle = task::spawn(async move {
            2.0*3.1415*self_divide.radius 
        });
        join_handle.await
    }

}
fn main() {
    task::block_on(async {
        let by_arc = Arc::new(Circle{radius:30.5});
        println!("{}", by_arc.area().await);
    })
}
