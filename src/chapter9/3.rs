// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // 使用 `Self` 填空
//     pub fn show_state(__)  {
//         println!("the current state is {}", self.color);
//     }
//
//     // 填空，不要使用 `Self` 或其变体
//     pub fn change_state() {
//         self.color = "green".to_string()
//     }
// }
// fn main() {}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 使用 `Self` 填空
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }

    // 填空，不要使用 `Self` 或其变体
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn main() {
    let light = TrafficLight {
        color: "red".to_owned(),
    };
    light.show_state();
    let mut light = light;
    light.change_state();
    println!("{:?}", light);
}
