extern crate ndarray;
//extern crate ndarray_linalg;

use ndarray::*;
//use ndarray_linalg::*;

fn main() {

    //2d 3x3矩阵相加
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6],
                   [7,8,9]]);

    let b = arr2(&[[9, 8, 7],
                   [6, 5, 4],
                   [3,2,1]]);
   //element-wise operation
    let sum = &a + &b;

    println!("\nmatrix add: \n{}\n + \n{}\n = \n{}\n", a, b, sum);

    //2d 3x3 矩阵相减
    //element-wise operation
    let sub = &a - &b;
    println!("\nmatrix sub: \n{}\n + \n{}\n = \n{}\n", a, b, sub);

    //2d 3x3 matrix product 当a and b 是2d矩阵时， dot执行“矩阵积”，即结果是一个矩阵。
    let dot_result_matrix = &a.dot(&b);
    println!("\nmatrix  product: \n{}",dot_result_matrix );
    //element-wise operation
    let dot_result_matrix = &a * &b;
    println!("\nmatrix element-wise multiplication: \n{}",dot_result_matrix );

    //1d 矩阵，即是一个向量，具有大小和方向
    //dot product
    let b_v: Array1<u16> = arr1(&[1,2,3]);
    let a_v: Array1<u16> = arr1(&[4,5,6]);
    //点积（内积,数量积）
    let dot_product = a_v.dot(&b_v);
    println!("\nmatrix dot product: \n{}",dot_product );
    //叉积(外积，向量积)
    //let outer_product = a_v.outer(&b_v);
    //println!("\nmatrix outer product: \n{}",outer_product );

    //element-wise operation
    let product = &a_v * &b_v;
    println!("\nmatrix element-wise multiplication: \n{}",product );
    //element-wise operation
    let div = &a_v / &b_v;
    println!("\nmatrix element-wise div: \n{}",div );
}
