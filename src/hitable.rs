use material::{Material};
use math::*;


#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    #[inline]
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<f32> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = (b * b) - (a * c);
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(temp);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(temp);
            }
        }
        None
    }
}
