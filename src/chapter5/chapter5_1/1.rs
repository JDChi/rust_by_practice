// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = String::from("hello, world");
//     let y = x;
//     println!("{},{}",x,y);
// }

// 因为 String 是存储在堆上，如果直接把 x 赋予给 y，其实就是转移了所有权，
// 这里其实我们想要的是深拷贝的效果
// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = String::from("hello, world");
//     let y = x.clone();
//     println!("{},{}", x, y);
// }

// 也可以让 x 不对字符串持有所有权，即它本身也只是引用
fn main() {
    // 使用尽可能多的方法来通过编译
    let x: &str = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}
