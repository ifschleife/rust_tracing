extern crate cgmath;
mod color;
mod ray;

use cgmath::*;
use std::f32;

use color::{Rgb};
use ray::{Ray};


fn color(r: Ray) -> Rgb {
    let direction = r.direction.normalize();
    let t = 0.5*(direction.y + 1.0);
    let color = (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0);

    Rgb::from(color)
}

fn main() {
    const NX: u32 = 200;
    const NY: u32 = 100;

    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let origin = vec3(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n255\n", NX, NY);
    for y in (0..NY).rev() {
        for x in 0..NX {
            let u = x as f32 / NX as f32;
            let v = y as f32 / NY as f32;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let color = color(r);
            let r = (255.99 * color.r) as u32;
            let g = (255.99 * color.g) as u32;
            let b = (255.99 * color.b) as u32;
            println!("{} {} {}", r, g, b);
        }
    }
}
