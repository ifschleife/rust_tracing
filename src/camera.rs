use rand::prelude::*;
use std::f32;

use math::*;


pub struct Camera {
    origin: Vec3f,
    lower_left_corner: Vec3f,
    horizontal: Vec3f,
    vertical: Vec3f,
    u: Vec3f,
    v: Vec3f,
    lens_radius : f32,
}

impl Camera {
    pub fn new(look_from: Vec3f, look_at: Vec3f, vup: Vec3f, vert_fov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vert_fov * f32::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = normalize(look_from - look_at);
        let u = normalize(cross(vup, w));
        let v = cross(w, u);
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

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut SmallRng) -> Ray {
        let rd = self.lens_radius * Camera::random_point_in_unit_disk(rng);
        let offset = self.u*rd.x + self.v*rd.y;
        return Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset);
    }

    fn random_point_in_unit_disk(rng: &mut SmallRng) -> Vec3f {
        loop {
            let p = 2.0f32*vec3f(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - vec3f(1.0, 1.0, 0.0);
            if dot(p, p) < 1.0 {
                return p;
            }
        }
    }
}
