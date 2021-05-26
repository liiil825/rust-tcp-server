pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    pub fn time (&self) -> u8 {
        match &self {
            TrafficLight::Red => 40,
            TrafficLight::Green => 60,
            TrafficLight::Yellow => 5,
        }
    }
}
