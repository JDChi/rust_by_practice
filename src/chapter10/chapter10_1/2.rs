//
// // 实现下面的泛型函数 sum
// fn sum
//
// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
// }

// 实现下面的泛型函数 sum
// 因为 T 的类型不一定能相加，所以要需要 std::ops::Add<Output = T>> 来对 T 进行限制
fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}
