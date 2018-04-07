use std::ops::{Add, AddAssign};


pub struct Vec3 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
        lhs.x*rhs.x + lhs.y*rhs.y + lhs.z*rhs.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
