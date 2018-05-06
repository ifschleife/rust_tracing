extern crate cgmath;
extern crate image;
mod hitable;
mod ray;

use cgmath::*;
use hitable::{Hitable, World};
use ray::{Ray};
use std::f32;


fn color(ray: Ray, world: &World) -> Vector3<f32> {
    match world.hit(&ray, 0.0, f32::MAX) {
        Some(record) => return 0.5*vec3(record.normal.x+1.0, record.normal.y+1.0, record.normal.z+1.0),
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5*(unit_direction.y+1.0);
            return (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    const NX: u32 = 200;
    const NY: u32 = 100;
    const BUFFER_SIZE: usize = (NX*NY*3) as usize;

    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let origin = vec3(0.0, 0.0, 0.0);

    let mut world = World{objects: Vec::new()};
    world.objects.push(Hitable::Sphere{center: vec3(0.0, 0.0, -1.0), radius: 0.5});
    world.objects.push(Hitable::Sphere{center: vec3(0.0, -100.5, -1.0), radius: 100.0});

    let mut buffer = Vec::with_capacity(BUFFER_SIZE);

    for y in (0..NY).rev() {
        for x in 0..NX {
            let u = x as f32 / NX as f32;
            let v = y as f32 / NY as f32;
            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let color = color(ray, &world);

            buffer.push((255.99 * color[0]) as u8);
            buffer.push((255.99 * color[1]) as u8);
            buffer.push((255.99 * color[2]) as u8);
        }
    }

    image::save_buffer("image.png", &buffer, NX, NY, image::RGB(8)).unwrap();
}
