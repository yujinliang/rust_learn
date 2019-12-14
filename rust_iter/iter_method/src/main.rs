

fn main() {

//几个Iterator常用的方法，背熟用好，非常有利于写出简洁代码。
    let v = vec![1,2,3,4,5,6];
    let it = v.iter().filter(|&x|{ x % 2 == 0});
    for x in it {
        println!("{}", x);
    }
//-----------------------------
    let collect: Vec<i32> =  v.iter().map( |&x|{ x*2}).collect();
    for x in collect {
        println!("{}", x);
    }
//-------------------------------
    let sum = v.iter().fold(0, | acc, &x|{ acc +x });
        println!("{}", sum );
//--------------------------------
   let v2 = vec![7,8,9,10];
   let all_it = v.iter().chain(v2.iter());
   for x in all_it {
           println!("{}", x);
   }
//--------------------------------
   let mut  idx_value_pair = v.iter().enumerate();
   println!("{:?}", idx_value_pair.next());
//-----------------------------
   println!("{}", v.iter().count());
//------------------------------
   let sum: i32 = v.iter().sum();
    println!("{}", sum);
//-------------------------------
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
for x in flattened {
     println!("{}", x);
}
//--------------------------------
  v.iter().for_each(|x|{ println!("{}",x);});
//-------------------------------
 let  the_item=  v.iter().nth(1);
 println!("{:?}", the_item);
//------------------------------
 let rev_it = v.iter().rev();
 for x in rev_it {
     println!("{}",x);
 }
//--------------------------------
 let a = [0, 1, 2, 3, 4, 5];
 let step_it = a.iter().step_by(2);
 for x in step_it {
     println!("{}", x);
 }
//-------------------------------

}
