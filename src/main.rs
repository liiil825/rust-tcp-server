mod homework;
use crate::homework::lesson4::traffic_light::TrafficLight;
use crate::homework::lesson4::summary::get_summary;
use crate::homework::lesson4::summary::MAX;

fn main() {
    // create_server();
    let lights = TrafficLight::Green;
    println!("time: {}", lights.time());

    let array = vec![1, 2, 3, 9];
    let summary = get_summary(&array);
    println!("{}", summary.unwrap());

    let maxArray = vec![1, 2, 3, MAX];
    let summary = get_summary(&maxArray);
    println!("{}", summary.unwrap_or_default());
}
