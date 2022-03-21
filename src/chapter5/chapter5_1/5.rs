// // 不要使用 clone，使用 copy 的方式替代
// fn main() {
//     let x = (1, 2, (), "hello".to_string());
//     let y = x.clone();
//     println!("{:?}, {:?}", x, y);
// }

// 不要使用 clone，使用 copy 的方式替代
fn main() {
    // x.3 是在堆上，所以在这个元组里可以直接 copy
    let x = (1, 2, (), &("hello".to_string()));
    // let y = (x.0, x.1, x.2, x.3);
    let y = x;
    println!("{:?}, {:?}", x, y);
}
