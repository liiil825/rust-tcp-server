use std::f64::consts::PI;

pub trait Area {
    fn get_area(&self) -> f64;
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

pub struct Square {
    pub side: f64,
}

pub struct Circle {
    pub radius: f64,
}

impl Area for Triangle {
    fn get_area(&self) -> f64 {
        return self.base * self.height * 0.5;
    }
}

impl Area for Square {
    fn get_area(&self) -> f64 {
        return self.side * self.side;
    }

}

impl Area for Circle {
    fn get_area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

pub fn get_area<T: Area>(geometry: &T) -> f64 {
    geometry.get_area()
}

pub enum Geometry {
    Triangle(f64, f64),
    Square(f64),
    Circle(f64),
}

impl Geometry {
    pub fn get_area(&self) -> f64 {
        match &self {
            Geometry::Triangle(x, y) => x * y * 0.5,
            Geometry::Square(x) => x * x,
            Geometry::Circle(r) => PI * r * r,
        }
    }
}
