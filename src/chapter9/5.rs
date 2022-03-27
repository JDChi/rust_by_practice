//
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// // 使用多个 `impl` 语句块重写下面的代码
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
//
// fn main() {}

struct Rectangle {
    width: u32,
    height: u32,
}

// 使用多个 `impl` 语句块重写下面的代码
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    let area = rect.area();
    println!("area is {}", area);
    let can_hold = rect.can_hold(&Rectangle {
        width: 20,
        height: 40,
    });
    println!("can hold = {}", can_hold);
}
