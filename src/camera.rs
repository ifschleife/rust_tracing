extern crate rand;
use cgmath::*;
use rand::Rng;
use std::f32;

use ray::{Ray};


pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    lens_radius : f32,
}

impl Camera {
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, vup: Vector3<f32>, vert_fov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vert_fov * f32::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w,
            horizontal: 2.0*half_width*focus_dist*u,
            vertical: 2.0*half_height*focus_dist*v,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Camera::random_point_in_unit_disk();
        let offset = self.u*rd.x + self.v*rd.y;
        return Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset);
    }

    fn random_point_in_unit_disk() -> Vector3<f32> {
        let mut rng = rand::weak_rng();

        loop {
            let p = 2.0f32*vec3(rng.next_f32(), rng.next_f32(), 0.0) - vec3(1.0, 1.0, 0.0);
            if p.dot(p) < 1.0 {
                return p;
            }
        }
    }
}
