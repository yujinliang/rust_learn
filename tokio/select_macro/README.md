# 			Rust [tokio](https://docs.rs/tokio/0.2.13/tokio/index.html)::select学习杂记

1. **前言**

Linux系统有select/poll/epoll等，主要用于监控各种fd上发生的各种event, 从而识别派发处理。golang语言中也有一个select，作用相似，主要监控channel上发生的可读可写event。 对于rust tokio/async_std/crossbeam/futures等也需要一个select去统一集中监控， 本笔记只针对tokio, 所以专门学习tokio crate提供的select!宏。

> 官方文档开篇对select!的定义：
>
> Wait on multiple concurrent branches, returning when the **first** branch completes, cancelling the remaining branches.



> 本人水平和精力有限，加之考证不详，故此难免谬误，粗鄙杂记随笔，只做抛砖引玉之举，切望见谅！



2. 要点

   

   （1）The `select!` macro must be used inside of async （functions, closures, and blocks）.

      (2)   每一个async expression and handler code 都是在当前task中执行的， 一旦block or long running 当前task所在的thread, 则select!没法检查其他branch case了！故此需避免此种情况，也可以调用tokio::spawn去并行执行，然后把join handle交给select!去监控即可。

   （3）else branch是必须的，可以避免当所有branch disable时, select! panic.

   > select!` panics if all branches are disabled **and** there is no provided `else` branch. A branch is disabled when the provided `if` precondition returns `false` **or** when the pattern does not match the result of `.

     (4) select!聚合所有enable branch的async expression并发执行， 一旦有执行完毕返回者， 则立即进行pattern模式匹配， 若匹配成功， 则执行handler code

   （5）select!文档开篇就对其有明确的定义，等待所有branch并发执行， 当第一个branch完成时，取消剩余branch async expression的执行！这就产生一个问题，如果你的async expression磨磨唧唧block/long running在那，不及时执行完毕返回，一旦其他branch首先执行完毕返回， 则select!首先模式匹配之， 一旦成功， 则本轮其他未执行完毕的async expression则被取消，最终导致这个branch一直不会成功，就像不存在！所以timeout那样持续性的future不适合用select!检测，selecct!拒绝他！还有async expression和handler code必须是那种即刻执行完毕返回的代码块，不可以sleep/delay/timeout/wait some thing/long runing等等， 因为他会剥夺select!检查其他branch的机会！

   （6）切记区分“并发”和“并行”的不同！select!只是“并发”执行branch，并非"并行".

   

3. 使用模式

   ```rust
   loop {
       //每轮loop遍历重新评估每一个branch 所以一个branch不会一直disable.
       tokio::select! {
           <pattern> = <async expression> (, if <precondition>)? => {
               //handler code 
               //the pattern failed or the precondition return false , then the branch 被认为disable branch
           },
           //...
            else => {println!("not match");},
           //当所有branch都是disable branch时， select!去执行else branch, 若是没有else branch , 则panic.
       }
   }
   ```

   > precondition 若为false, 则disable 此branch case,  but async expression is still evaluated, but the resulting future is not polled.大意为：只是评估async expression得出一个future, 但是不会真正去执行这个future. precondition 若为true, 则正常run 此future.

   >  pattern 用于匹配async expression.await的执行结果.

   > async expression 一般代表一个可以后缀.await来实际执行的代码块，如async fn/block等.

4. select! 完整执行流程

   > （1）Evaluate all provided precondition expressions. If the precondition returns false, disable the branch for the remainder of the current call to select!. Re-entering select! due to a loop clears the "disabled" state.
   > （2）Aggregate the async expressions from each branch, including the disabled ones. If the branch is disabled, async expression is still evaluated, but the resulting future is not polled.
   > （3）Concurrently await on the results for all remaining async expressions.
   > （4）Once an async expression returns a value, attempt to apply the value to the provided pattern, if the pattern matches, evaluate handler and return. If the pattern does not match, disable the current branch and for the remainder of the current call to select!. Continue from step 3。
   > （5）If all branches are disabled, evaluate the else expression. If none is provided, panic.
   >
   > 详情请参看： https://docs.rs/tokio/0.2.13/tokio/macro.select.html

5. code example

   ```rust
   use tokio::time::{self, Duration,delay_for,timeout};
   use tokio::stream::{self, StreamExt};
   use tokio::sync::{oneshot,mpsc,broadcast};
   use tokio::task;
   
   async fn some_computation(input: u32) -> String {
       format!("the result of computation {}", input)
   }
   
   async fn some_async_work() {
       // do work
       //delay_for(Duration::from_millis(100000)).await;
       //只需注释掉上一行代码，并追加一行无线循环代码， 即可验证select!在当前同一个task所在的thread中并发执行
       //所有<async expression>, 一旦当前thread被block住，则select!不能再check其他branch的<async expression>了
       //所以切记<async expression>中不要有block当前线程的代码！
   }
   
   #[tokio::main]
   async fn main() {
       //time::delay
       let mut delay = time::delay_for(Duration::from_millis(5));
       //stream
       let mut stream1 = stream::iter(vec![1, 2, 3]);
       //oneshot
       let (tx1, mut rx1) = oneshot::channel();
       tokio::spawn(async move {
           tx1.send("first").unwrap();
       });
       let mut a = None;
       //mpsc
       let (mut tx2, mut rx2) = mpsc::channel(100);
       tokio::spawn(async move {
           for i in 0..10 {
               let res = some_computation(i).await;
               tx2.send(res).await.unwrap();
           }
       });
       let mut done = false;
       //broadcast 
       let (tx3, mut rx3) = broadcast::channel(16);
       let mut rx4 = tx3.subscribe();
       tx3.send(10).unwrap();
       tx3.send(20).unwrap();
       tokio::spawn(async move {
           assert_eq!(rx4.recv().await.unwrap(), 10);
           assert_eq!(rx4.recv().await.unwrap(), 20);
       });
       //time::interval
       let mut interval = time::interval(Duration::from_millis(2));
       //join handle
       let mut join_done = false;
       let mut join_handle: task::JoinHandle<u8> = task::spawn(async {
           // some work here
           delay_for(Duration::from_millis(1)).await;
           88
       });
       //time::timeout
       //let mut to = timeout(Duration::from_millis(5), some_async_work());
   
       loop {
           tokio::select! {
               _ = &mut delay => {
                   println!("delay reached");
                   break;
               },
              /* _ = &mut to => {
                   println!("operation timed out");
                   break;
               },*/
               ret_code=&mut join_handle ,if !join_done => {
                   join_done = true;
                   println!("join handle case: {:?}", ret_code);
               },
               _= interval.tick() => {
                   println!("operation interval");
               },
               _ = some_async_work() => {
                   println!("operation completed");
                   //delay_for(Duration::from_millis(100000)).await;
                   //此处加上delay_for可用于验证， <handler code>一旦有block/long running 当前所在task的代码
                   //则select!无法再去check其他branch了， 所以切记避免之！！！
               },
               Some(v) = stream1.next() => { println!("stream: {}", v);},
               v1 = (&mut rx1), if a.is_none()  =>  {
                   println!("oneshot : {:?}", v1);a = v1.ok();
               },
               v2 = rx2.recv(), if !done  => {
                   println!("mpsc: {:?}", v2);
                    if v2.is_none() { done = true; }
               },
               v3 = rx3.recv() => {
                   println!("broadcast: {:?}", v3);
               },
               else => {
                   println!("not match");
               },
           }
       }
   }
   ```

   

   6. 后记

   tokio::select!和golang::select还是有很大不同的， 后者主要监控channel, 而前者用于监控async expression，不限于channel！也不是监控channel的可读可写状态！不同于一般意义上的io event poll, 本质上讲select!就是每次同时并发执行所有enabel branch的async expression.await，一旦其中某个有结果，则接着对结果执行模式匹配， 成功了则执行handler code.

   > 现在tokio::select!的编译报错相当不友好，一处出错整体泛红，令人无从下手。个人体会其可用性易用性和友好性远不如golang::select,确实需要打磨。

   

   7. 疑问

      > (1) tokio::select!只是描述接受async expression， 但是实验发现并非所有的async expression都被接受，比如：tokio::time::timeout, 我是在rust stable 1.42版本测试的， 有时间我在慢慢研究吧。
      >
      > (2)tokio::select!对于每一个branch case, 其实主要检测async expression.await是否执行返回， 那么对于channel 而言，容易检测已读已写！ 对于可读可写， 固然可以通过检查channel的len(), is_full(), is_empty()来判断， 但是当handler code被执行时，之前的判断很可能已经不成立！产生race condition问题，不知是否送多虑了？？？
      >
      > (3) 虽然其文档中明确描述select!随机挑选一个check, 但参看其文档中的执行流程，分明是先来先得， 即那个async expression先执行完毕返回，select!就先check它， 从实际测试代码的执行输出来看也体现如此！比较扎堆， async expression的并发随机执行由tokio::runtime::executor来保证， 但是select!文档却说随机挑选一个branch进行check ! 故此对其随机性和公平性我却有些疑惑了！

   8. 参考资料

   https://docs.rs/tokio/0.2.13/tokio/macro.select.html

   

   9. test  code: https://github.com/yujinliang/rust_learn/tree/master/tokio/select_macro

