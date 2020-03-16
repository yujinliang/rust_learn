use async_std::task;

struct Circle {
    radius:f64
}
impl Circle {
    async fn area(&self) -> f64 {
        task::block_on(async {
            2.0*3.1415*self.radius 
        }) 
    }

}
fn main() {
    task::block_on(async {
        let by_blockon = Circle{radius:30.5};
        println!("{}", by_blockon.area().await);
    })
}
