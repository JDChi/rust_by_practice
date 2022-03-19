
// fn main() {
//     assert!(0.1+0.2==0.3);
// }


fn main() {
    assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
    assert!((0.1_f64  + 0.2 - 0.3).abs() < 0.0001);
}

