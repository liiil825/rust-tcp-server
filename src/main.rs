mod homework;
use crate::homework::lesson4::traffic_light::TrafficLight;

fn main() {
    // create_server();
    let lights = TrafficLight::Green;
    println!("time: {}", lights.time());
}
