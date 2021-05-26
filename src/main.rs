mod homework;
use crate::homework::lesson4::traffic_light::TrafficLight;
use crate::homework::lesson4::summary::{get_summary, MAX};
use crate::homework::lesson4::area::{Geometry};
use crate::homework::lesson4::area::{Triangle, Square, Circle, get_area};

fn main() {
    // create_server();
    let light = TrafficLight::Red;
    println!("time: {}", light.time());
    let light = TrafficLight::Green;
    println!("time: {}", light.time());
    let light = TrafficLight::Yellow;
    println!("time: {}", light.time());

    let array = vec![1, 2, 3, 9];
    let summary = get_summary(&array);
    println!("{}", summary.unwrap());

    let array = vec![1, 2, 3, MAX];
    let summary = get_summary(&array);
    println!("{}", summary.unwrap_or_default());

    let triangle = Geometry::Triangle(3.0, 4.0);
    println!("The area of this triangle is {}", triangle.get_area());
    let square = Geometry::Square(3.0);
    println!("The area of this square is {}", square.get_area());
    let circle = Geometry::Circle(3.0);
    println!("The area of this circle is {}", circle.get_area());

    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    println!("The area of this triangle is {}", get_area(&triangle));
    let square = Square {
        side: 3.0,
    };
    println!("The area of this square is {}", get_area(&square));
    let circle = Circle {
        radius: 3.0,
    };
    println!("The area of this circle is {}", get_area(&circle));
}
