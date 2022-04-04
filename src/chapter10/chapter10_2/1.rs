//
// // 修复错误
// struct Array<T, const N: usize> {
//     data : [T; N]
// }
//
// fn main() {
//     let arrays = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1.0, 2.0, 3.0],
//         },
//         Array {
//             data: [1, 2]
//         }
//     ];
// }

// 修复错误
struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    // <T, const N: usize> 是定义给 Array 的，又因为数组的规定是类型相同，
    // 所以这里想让 Array 相同，就要让 Array 里的 data 在类型和长度相同
    let arrays = [
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 3] },
    ];
}
