// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }
//
// // 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T>(x: T, y: T) -> T {
//     x + y
// }

use std::ops::Add;

fn main() {
    assert_eq!(sum(1, 2), 3);
}

// 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// 通过两种方法使用特征约束来实现 `fn sum`
fn sum<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}
