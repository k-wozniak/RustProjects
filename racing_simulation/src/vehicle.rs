#[derive(Debug)]
pub struct Vehicle {
    pub name: String,
    pub number: i32,
    pub acceleration: f32,  // ms-2
    pub max_speed: f32,     // ms-1
}