use std::f32;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub};

pub fn vec3f(x: f32, y: f32, z: f32) -> Vec3f {
    Vec3f::new(x, y, z)
}

pub fn cross(vec1: Vec3f, vec2: Vec3f) -> Vec3f {
    Vec3f {
        x: (vec1.y * vec2.z) - (vec1.z * vec2.y),
        y: (vec1.z * vec2.x) - (vec1.x * vec2.z),
        z: (vec1.x * vec2.y) - (vec1.y * vec2.x),
    }
}

pub fn dot(vec1: Vec3f, vec2: Vec3f) -> f32 {
    (vec1.x * vec2.x) + (vec1.y * vec2.y) + (vec1.z * vec2.z)
}

pub fn normalize(vec: Vec3f) -> Vec3f {
    let length = vec.length();
    vec3f(vec.x / length, vec.y / length, vec.z / length)
}

#[derive(Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn zero() -> Vec3f {
        Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Vec3f {
        Vec3f {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x, y, z }
    }

    pub fn length(self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(self) -> f32 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        *self = vec3f(self.x / length, self.y / length, self.z / length)
    }
}

impl Add<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3f {
    fn add_assign(&mut self, other: Vec3f) {
        *self = vec3f(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Div<f32> for Vec3f {
    type Output = Vec3f;
    fn div(self, other: f32) -> Vec3f {
        vec3f(self.x / other, self.y / other, self.z / other)
    }
}

impl DivAssign<f32> for Vec3f {
    fn div_assign(&mut self, other: f32) {
        *self = vec3f(self.x / other, self.y / other, self.z / other)
    }
}

impl Mul<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, other: f32) -> Vec3f {
        Vec3f {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;
    fn neg(self) -> Vec3f {
        vec3f(-self.x, -self.y, -self.z)
    }
}

impl Sub<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn sub(self, other: Vec3f) -> Vec3f {
        vec3f(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_time(&self, t: f32) -> Vec3f {
        self.origin + t*self.direction
    }
}
