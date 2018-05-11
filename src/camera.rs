use cgmath::*;
use ray::{Ray};
use std::f32;


pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
}

impl Camera {
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, vup: Vector3<f32>, vert_fov: f32, aspect: f32) -> Camera {
        let theta = vert_fov * f32::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width*u - half_height*v - w,
            horizontal: 2.0*half_width*u,
            vertical: 2.0*half_height*v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin);
    }
}
