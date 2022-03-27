//
// // 修复错误，尽量少地修改代码
// // 不要移除任何代码行
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;
//
//     match r {
//         &mut value => value.push_str(" world!")
//     }
// }

fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    let result = match r {
        value => {
            value.push_str(" world!");
            value
        }
    };
    println!("{}", result);
}
