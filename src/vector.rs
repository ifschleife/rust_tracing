extern crate cgmath;
use cgmath::*;


pub trait VectorLength {
    fn length(&self) -> f32;
    fn squared_length(&self) -> f32;
}

impl VectorLength for Vector3<f32> {
    fn length(&self) -> f32 {
        return self.squared_length().sqrt();
    }
    fn squared_length(&self) -> f32 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }
}
