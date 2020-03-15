# 			Rust [tokio](https://docs.rs/tokio/0.2.13/tokio/index.html)::select学习杂记

1. **前言**

Linux系统由select/poll/epoll等，主要用于监控各种fd上发生的各种event, 从而识别派发处理。golang语言中也有一个select，作用相似，也是主要监控channel上发生的可读可写event。 对于rust tokio/async_std/crossbeam/futures等都提供了和golang相似的channel, 所以必然也需要一个select去统一集中监控之， 本笔记只针对tokio, 所以专门学习tokio crate提供的select!宏。



2. 要点

   （1）The `select!` macro must be used inside of async （functions, closures, and blocks）.

     (2)   每一个<async expression> and handler code 都是在当前task中执行的， 一旦block or long running 		  则select!没法检查其他branch case了！故此需避免此种情况，也可以调用tokio::spawn去并行执行，然		  后把join handle交给select!去监控即可。

   （3）else branch是必须的，可以避免当所有branch disable时, select! panic.

   > `select!` panics if all branches are disabled **and** there is no provided `else` branch. A branch is disabled when the provided `if` precondition returns `false` **or** when the pattern does not match the result of `.

     (4) select!每次随机挑选一个branch检查， 通常和loop一个配合使用.

   

3. 使用模式

   ```rust
   loop {
       tokio::select! {
           <pattern> = <async expression> (, if <precondition>)? => {
               //handler code 
           },
           //...
            else => {println!("not match");},
       }
   }
   ```

   > <precondition> 若为false, 则disable 此branch case,  but `async expression>` is still evaluated, but the resulting future is not polled.大意为：只是评估<async expression>得出一个future, 但是不会真正去执行这个future. <precondition> 若为true, 则正常run 此branch case.

   >  <pattern> 用于匹配<async expression>.await的执行结果.

   > <async expression> 一般代表一个可以后缀.await来实际执行的代码块，如async fn/block等.

4. select! 完整执行流程

   > 1. Evaluate all provded `` expressions. If the precondition returns `false`, disable the branch for the remainder of the current call to `select!`. Re-entering `select!` due to a loop clears the "disabled" state.
   > 2. Aggregate the ``s from each branch, including the disabled ones. If the branch is disabled, `` is still evaluated, but the resulting future is not polled.
   > 3. Concurrently await on the results for all remaining ``s.
   > 4. Once an `` returns a value, attempt to apply the value to the provided ``, if the pattern matches, evaluate `` and return. If the pattern **does not** match, disable the current branch and for the remainder of the current call to `select!. Continue from step 3.
   > 5. If **all** branches are disabled, evaluate the `else` expression. If none is provided, panic.

5. code example

   ```rust
   use tokio::time::{self, Duration,delay_for};
   use tokio::stream::{self, StreamExt};
   use tokio::sync::{oneshot,mpsc,broadcast};
   
   async fn some_computation(input: u32) -> String {
       format!("the result of computation {}", input)
   }
   
   async fn some_async_work() {
       // do work
       delay_for(Duration::from_millis(1)).await;
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
       
       loop {
           tokio::select! {
               _ = &mut delay => {
                   println!("operation timed out");
                   break;
               },
               _= interval.tick() => {
                   println!("operation interval");
               },
               _ = some_async_work() => {
                   println!("operation completed");
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

   6. 疑问

   1. select!都接受或拒绝那些<async expression>， 因为实际代码测试发现，不是所有的<async expression>都被接受？？？？？

   2. select!如何监控join handle？？？

   3. 对于channel 的可读可写， 读完毕、写完毕的监控, select!如何实现？？？
   
   4. select!可以check delay , 可为什么不可以check timeout, 仿照delay branch case 加入timeout branch case 则编译报错！为什么？？？
   
      答: timeout 若作为select!的branch case, 肯定block or long running , 导致select!当前所属的task被block住，从而导致select!不再可能check其他branch case了， 知道timeout发生？！？？？？
   
      
   
      7. 后记
   
   tokio::select!和golang::select还是有很大不同的， 后者主要监控channel, 而前者用于监控<async expression>，不限于channel！也不是监控channel的可读可写状态！不同于一般意义上的io event poll, 本质上讲select!就是每次随机选一个branch case检查， 看看async expression>.await是否执行返回，一旦有结果，则接着对结果执行模式匹配， 成功了最后执行handler code.
   
   > 现在tokio::select!的编译报错相当不友好，一处出错整体泛红，令人无从下手。
   
   

8. 参考资料

   https://docs.rs/tokio/0.2.13/tokio/macro.select.html

   

