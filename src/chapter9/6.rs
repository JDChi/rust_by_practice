#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// 为 TrafficLightColor 实现所需的方法
impl TrafficLightColor {
    fn color(&self) -> String {
        match self {
            TrafficLightColor::Yellow => "yellow".to_string(),
            _ => "nothing".to_string(),
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}
