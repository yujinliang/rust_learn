# 								`Atomic Memory Order 杂记`

- 原子操作(atomic)

> 所谓原子操作是指不会被线程调度机制打断的操作；这种操作一旦开始，就一直运行到结束，中间不会有任何 context switch （切换到另一个线程）。
> 原子操作可以是一个步骤，也可以是多个操作步骤，但是其顺序不可以被打乱，也不可以被切割而只执行其中的一部分。
> 将整个操作视作一个整体是原子性的核心特征。
>
> 处理器提供总线锁定和缓存锁定两个机制来保证复杂内存操作的原子性.
>
> 原子性不可能由软件单独保证--必须需要硬件的支持，因此是和架构相关的。
>
> 编译器优化而产生的指令乱序，`cpu`指令流水线也会产生指令乱序，总体来讲，编译器优化层面会产生的指令乱序，`cpu`层面也会的指令流水线优化产生乱序指令。当然这些乱序指令都是为了同一个目的，优化执行效率。
>
> `https://doc.rust-lang.org/nomicon/atomics.html`



- `Happen-before` 和 `Synchroize-with`

> 【Happens-Before】 
> 	这是现代编程语言中很常见的一个术语，每个语言在它们的 specifications 中都会有这个关系的定义和描述。可以这样来简单阐释这个词：
>
> ​	A、B 是两个在多核 CPU 上执行的操作。如果 A happens-before B，那么 A 所产生的内存变化会在 B 操作执行之前被看到（visible）。
> ​	不管我们使用什么编程语言，在同一个线程下的顺序语句总是遵循 happens-before 原则的。
>
> 【Synchronizes-with】
>
>    简单来说，两个线程 A 和 B，以及一个支持原子操作的变量 x，如果 A 线程对 x 写入了一个新的值（store），而 B 线程在 x 上面读取到了这个新的值（load），我们就可以认为，A 的 store 就是 synchronizes-with B 的 load 的。
>
> ​	对于跨线程（inter-thread） 的情况，要判断 happens-before，就需要借助于前面提到的 synchronizes-with 了。如果操作 A 是 synchronizes-with 另一	个线程的操作 B 的，那么 A 就是 happens-before B 的。Happens-before 也具有传递性，如果 B 是 happens-before C 的，那么 A 也是 happens-before 	C。
>
> 好文章我推荐：`https://www.jianshu.com/p/511cde6b62a6` ， `https://www.cnblogs.com/ishen/p/13200838.html`





- Memory Barrier

> 内存栅栏是一个令 CPU (`CPU fence`)或编译器(`compiler fence`)在内存操作上限制内存操作顺序的指令，通常意味着在 barrier 之前的指令一定在 barrier 之后的指令之前执行。
>
> 栅栏相当于给内存加了一层栅栏，约束内存乱序。典型用法是和 relaxed一起使用。 ***栅栏属于全局操作， 执行栅栏操作可以影响到在线程中的其他原子操作。***
>
> 一般来说memory fence分为两层：`compiler fence`和`CPU fence`，前者只在编译期生效，目的是防止compiler生成乱序的内存访问指令；后者通过插入或修改特定的CPU指令，在运行时防止内存访问指令乱序执行。
>
> 
>
> 好文章我推荐：`https://en.cppreference.com/w/cpp/atomic/atomic_thread_fence` ， `https://blog.csdn.net/wangdamingll/article/details/107024941` ，
>
> `https://blog.csdn.net/wxj1992/article/details/103917093` ， `https://zhuanlan.zhihu.com/p/43526907`





- Atomic memory order

> 【relaxed】
>
> 只保证当前操作的原子性，不考虑线程间的同步（读写顺序），其他线程可能读到新值，也可能读到旧值。
>
> 没有顺序一致性的要求，也就是说同一个线程的原子操作还是按照happens-before关系，但不同线程间的执行关系是任意。
>
> ```c++
> 
>              std::atomic<int> x = 0;     // global variable
>              std::atomic<int> y = 0;     // global variable
> 		  
> Thread-1:                                                                            Thread-2:
> r1 = y.load(memory_order_relaxed); // A             r2 = x.load(memory_order_relaxed); // C
> x.store(r1, memory_order_relaxed); // B              y.store(42, memory_order_relaxed); // D
> 
> ```
>
> 　　执行完上面的程序，可能出现`r1 == r2 == 42`。理解这一点并不难，因为编译器允许调整 C 和 D 的执行顺序。如果程序的执行顺序是 D -> A -> B -> C，那么就会出现`r1 == r2 == 42`。
>
> 好文章我推荐：`http://senlinzhan.github.io/2017/12/04/cpp-memory-order/`
>
> 

> 【release】（可以理解为` mutex 的 unlock 操作`）
>
> 1. 对**写入**施加 release 语义（store），在代码中这条语句前面的所有读写操作都无法被重排到这个操作之后，即 store-store 不能重排为 store-store, load-store 也无法重排为 store-load。
> 2. 当前线程内的**所有**写操作，对于其他对这个原子变量进行 acquire 的线程可见。
> 3. 当前线程内的**与这块内存有关**的**所有**写操作，对于其他对这个原子变量进行 consume 的线程可见。
>
> 
>
> 【acquire】（可以理解为` mutex 的 lock 操作`）
>
> 1. 对**读取**施加 acquire 语义（load），在代码中这条语句后面所有读写操作都无法重排到这个操作之前，即 load-store 不能重排为 store-load, load-load 也无法重排为 load-load
> 2. 在这个原子变量上施加 release 语义的操作发生之后，acquire 可以保证读到所有在 release 前发生的写入，比如：
>
> ```c++
> c = 0;
> 
> thread 1:
> {
>   a = 1;
>   b.store(2, memory_order_relaxed);
>   c.store(3, memory_order_release);  <-----------------|
> }																										|
>                                                                                                          |
> thread 2:                                                                                      |
> {                                                                                                       |
>   while (c.load(memory_order_acquire) != 3)  ------- | //满足规则2.  对a 和b的写也被一同同步过来了！ 
>     ;
>   // 以下 assert 永远不会失败
>   assert(a == 1 && b == 2);
>   assert(b.load(memory_order_relaxed) == 2);
> }
> ```
>
> 
>
> 【consume】
>
> 1. 对当前**要读取的内存**施加 release 语义（store），在代码中这条语句(指的是consume)后面所有**与这块内存有关的**读写操作都无法被重排到这个操作之前
> 2. 在这个原子变量上施加 release 语义的操作发生之后，consume 可以保证读到所有在 release 前发生的**并且与这块内存有关的**写入，举个例子：
>
> ```
> a = 0;
> c = 0;
> 
> thread 1:
> {
>   a = 1;
>   c.store(3, memory_order_release);
> }
> 
> thread 2:
> {
>   while (c.load(memory_order_consume) != 3)
>     ;
>   assert(a == 1); // assert 可能失败也可能不失败， 因为a与c之间无关， 但c=a+1; 则有关。
> }
> ```
>
> 通俗地讲，我只想同步一个 x 的读写操作，结果把 release 之前的写操作都顺带同步了？如果我想避免这个额外开销怎么办？用 release -- consume 呗。同步还是一样的同步，这回副作用弱了点：在线程 B acquire x 之后的读操作中，有一些是依赖于 x 的值的读操作。管这些依赖于 x 的读操作叫 赖B读. 同理在线程 A 里面, release x 也有一些它所依赖的其他写操作，这些写操作自然发生在 release x 之前了。管这些写操作叫 赖A写. 现在这个副作用就是，只有 赖B读 能看见 赖A写。 比如：
>
> ```c
> //数据依赖（carries dependency）
> 
> S1. c = a + b;
> S2. e = c + d;
> 
> //S2 数据依赖于 S1，因为它需要 c 的值。
> ```
>
> 好文章我推荐：`https://www.zhihu.com/question/24301047/answer/85844428     author:  zlilegion`
>
> 
>
> 【`acq_rel`】
>
> 1. 对读取和写入施加 acquire-release 语义，无法被重排
> 2. 可以看见其他线程施加 release 语义的所有写入，同时自己的 release 结束后所有写入对其他施加 acquire 语义的线程可见
> 3. 说白了，就是对store with release, load with acquire.
>
> 　　除此之外，还有另一种效果：假设 Thread-1 `store()`的那个值，成功被 Thread-2 `load()`到了，那么 Thread-1 在`store()`之前对内存的所有写入操作，此时对 Thread-2 来说，都是可见的。
>
> ```c++
> #include <thread>
> #include <atomic>
> #include <cassert>
> #include <string>
> std::atomic<bool> ready{ false };
> int data = 0;
> void producer()
> {
>     data = 100;                                       // A
>     ready.store(true, std::memory_order_release);     // B
> }
> void consumer()
> {
>     while (!ready.load(std::memory_order_acquire))    // C
>         ;
>     assert(data == 100); // never failed              // D
> }
> int main()
> {
>     std::thread t1(producer);
>     std::thread t2(consumer);
>     t1.join();
>     t2.join();
>     return 0;
> }
> ```
>
> 让我们分析一下这个过程：
>
> - 首先 A 不允许被移动到 B 的后面。
> - 同样 D 也不允许被移动到 C 的前面。
> - 当 C 从 while 循环中退出了，说明 C 读取到了 B `store()`的那个值，此时，Thread-2 保证能够看见 Thread-1 执行 B 之前的所有写入操作（也即是 A）。
>
> 
>
> 【`seq_cst顺序一致性`】
>
> 1. 如果是读取就是 acquire 语义，如果是写入就是 release 语义，如果是读取+写入就是 acquire-release 语义
> 2. 同时会对所有使用此 memory order 的原子操作进行同步，所有线程看到的内存操作的顺序都是一样的，就像单个线程在执行所有线程的指令一样
>
> 通常情况下，默认使用 `*memory_order_seq_cst`，所以你如果不确定怎么这些 memory order，就用这个。
>
> 好文章我推荐：`https://zhuanlan.zhihu.com/p/45566448` 、 `http://senlinzhan.github.io/2017/12/04/cpp-memory-order/`







【C++ atomic memory order】

```rust
//https://en.cppreference.com/w/cpp/atomic/memory_order
 namespace std {
      typedef enum memory_order {
      memory_order_relaxed, memory_order_consume, memory_order_acquire,
        memory_order_release, memory_order_acq_rel, memory_order_seq_cst
  } memory_order;

```



【Rust atomic memory order】

```rust
//https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

> ## Rust 中的顺序保证
>
> Rust 对于原子数据的访存提供了 4 种顺序（C++ 提供了 6 种），分别是 Sequentially Consistent、Release、Acquire、Relaxed。
>
> ### Sequentially Consistent
>
> 这是一种最强的内存栅栏，在这之前和之后的内存指令都不能跨越它。在 Rust 官方的 `nomicon 中建议没有特别强烈的需要使用 ``SeqCst` 和 `Relaxed` 就够了，运行稍慢总比出错好。有待验证研究！？
>
> 好文章：`https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust/`





- ### `volatile`

> `voldatile`关键字首先具有“易变性”，声明为volatile变量编译器会强制要求读内存，相关语句不会直接使用上一条语句对应的的寄存器内容，而是重新从内存中读取。
>
> 其次具有”不可优化”性，volatile告诉编译器，不要对这个变量进行各种激进的优化，甚至将变量直接消除，保证代码中的指令一定会被执行。
>
> 最后具有“顺序性”，能够保证Volatile变量间的顺序性，编译器不会进行乱序优化。不过要注意与非volatile变量之间的操作，还是可能被编译器重排序的。
>
> 需要注意的是其含义跟原子操作无关，比如：volatile int a; a++; 其中a++操作实际对应三条汇编指令实现”读-改-写“操作（`RMW`），并非原子的。
>
> 好文章我推荐：`https://zhuanlan.zhihu.com/p/43526907`





- `SpinLock`

> 使用原子量的` CAS` 操作 + loop 来实现 **Spin Lock**。
>
> ```c++
> //一个简单的自旋锁
> struct spinlock {
>  void lock() {
>      bool expected = false;
>      while (!state.compare_exchange_weak( //所谓的自旋锁其实就是加锁失败时不会导致blocking,  一直loop重试。
>              expected, true, std::memory_order_acquire, std::memory_order_relaxed)) {
>          expected = false;
>      }
>  }
> 
>  void unlock() {
>      state.store(false, std::memory_order_release);
>  }
> private:
>  std::atomic_bool state;
> };
> ```
>
> ```c++
> //最后再来看看SpinLock的实现，所有SpinLock都是基于原子操作进行的，目前我碰到的大致分为两种：
> 
> //比较无赖，强制for循环等待
> //比较友善，在超过一定循环次数，会放弃当前时间片
> //伪代码：
> atomic flag
> while flag:
>     loop_count++
>     if loop_count > MAX_LOOP_COUNT:
>         yield  // iOS中可以是thread_swtich
> ```
>
> 好文章我推荐：`https://zhuanlan.zhihu.com/p/31386431` , `https://www.jianshu.com/p/83f75ce281a2`



- lock-free 算法设计核心方法原则

> Lock-free programming has the following advantages:
>
> 1. Can be used in places where locks must be avoided, such as interrupt handlers
> 2. Avoid the troubles with blocking, such as deadlock and priority inversion
> 3. Improve performance on a multi-core processor
>
> lock-free编程实现的关键方法：
>
> 您可以独立地更新部分数据结构的本地副本，然后使用`CAS`（比较和交换）以原子方式将其应用于共享结构。（避免阻塞）



- Reference

> `https://baike.baidu.com/item/%E5%8E%9F%E5%AD%90%E6%93%8D%E4%BD%9C/1880992`
>
> `https://zhuanlan.zhihu.com/p/31386431`
>
> `https://zhuanlan.zhihu.com/p/45566448`
>
> `https://www.zhihu.com/question/24301047/answer/85844428   zlilegion`
>
> `https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html`
>
> `https://en.cppreference.com/w/cpp/atomic/memory_order`
>
> `https://www.cnblogs.com/ishen/p/13200838.html`
>
> `https://zhuanlan.zhihu.com/p/31386431`
>
> `https://hunterlxt.github.io/atomic-ordering/`
>
> `https://preshing.com/20120913/acquire-and-release-semantics/`
>
> `https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust/`
>
> `https://stackoverflow.com/questions/30407121/which-stdsyncatomicordering-to-use`
>
> `https://www.zhihu.com/question/24301047`
>
> `https://www.jianshu.com/p/511cde6b62a6`
>
> `https://zhuanlan.zhihu.com/p/31386431`
>
> `https://zhuanlan.zhihu.com/p/41872203`
>
> `https://en.cppreference.com/w/cpp/atomic/atomic_thread_fence`
>
> `https://blog.csdn.net/wangdamingll/article/details/107024941`
>
> `https://blog.csdn.net/wxj1992/article/details/103917093`
>
>  `https://www.jianshu.com/p/83f75ce281a2`
>
> `https://zhuanlan.zhihu.com/p/43526907`
>
> `https://doc.rust-lang.org/std/sync/atomic/fn.fence.html
> https://doc.rust-lang.org/std/sync/atomic/fn.compiler_fence.html`

