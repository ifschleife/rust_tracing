extern crate cgmath;
extern crate image;
mod ray;

use std::f32;
use cgmath::*;
use ray::{Ray};


fn hit_sphere(center: &Vector3<f32>, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-b - discriminant.sqrt()) / (2.0*a)
    }
}

fn color(r: Ray) -> Vector3<f32> {
    let t = hit_sphere(&vec3(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let normal = (r.point_at_time(t) - vec3(0.0, 0.0, -1.0)).normalize();
        return 0.5*(normal + vec3(1.0, 1.0, 1.0))
    }

    let direction = r.direction.normalize();
    let t = 0.5*(direction.y + 1.0);

    (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0)
}


fn main() {
    const NX: u32 = 200;
    const NY: u32 = 100;

    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let origin = vec3(0.0, 0.0, 0.0);

    let mut buffer = vec![0; (NX * NY * 3) as usize];

    for y in (0..NY).rev() {
        for x in 0..NX {
            let u = x as f32 / NX as f32;
            let v = y as f32 / NY as f32;
            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let color = color(ray);

            let index = (y*NX*3 + x*3) as usize;
            buffer[index + 0] = (255.99 * color[0]) as u8;
            buffer[index + 1] = (255.99 * color[1]) as u8;
            buffer[index + 2] = (255.99 * color[2]) as u8;
        }
    }

    image::save_buffer("image.png", &buffer, NX, NY, image::RGB(8)).unwrap();
}
