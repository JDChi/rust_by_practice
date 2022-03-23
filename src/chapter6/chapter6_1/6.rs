// // 修复所有错误，不要删除任何一行代码
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1 + s2;
//     assert_eq!(s3,"hello,world!");
//     println!("{}",s1);
// }

// my solution
fn main() {
    let s1 = "hello,";
    let mut s2 = String::from("world!");
    s2.insert_str(0, s1);
    let s3 = s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

// reference solution
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1.clone() + &s2;
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s1);
// }
