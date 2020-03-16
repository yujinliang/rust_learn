use async_std::task;

struct Circle {
    radius:f64
}
impl Circle {
    async fn area(&'static self) -> f64 {
       let j_handle = task::spawn(async move {
            2.0*3.1415*self.radius 
        }) ;
        j_handle.await
    }

}
fn main() {
    task::block_on(async {
        static BY_STATIC:Circle = Circle{radius:30.5};
        println!("{}", BY_STATIC.area().await);
        println!("{}", BY_STATIC.area().await);
    })
}
