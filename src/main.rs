extern crate cgmath;
extern crate image;
extern crate rand;
use cgmath::*;
use rand::Rng;
use std::f32;

mod camera;
mod hitable;
mod ray;
use camera::{Camera};
use hitable::{Hitable, World};
use ray::{Ray};


fn color(ray: &Ray, world: &World) -> Vector3<f32> {
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
    const NS: u32 = 100;
    const BUFFER_SIZE: usize = (NX*NY*3) as usize;

    let mut world = World{objects: Vec::new()};
    world.objects.push(Hitable::Sphere{center: vec3(0.0, 0.0, -1.0), radius: 0.5});
    world.objects.push(Hitable::Sphere{center: vec3(0.0, -100.5, -1.0), radius: 100.0});

    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    let mut buffer = Vec::with_capacity(BUFFER_SIZE);

    for y in (0..NY).rev() {
        for x in 0..NX {
            let mut col = vec3(0.0, 0.0, 0.0);
            for _ in 0..NS {
                let u = (x as f32 + rng.gen_range(0.0, 1.0)) / NX as f32;
                let v = (y as f32 + rng.gen_range(0.0, 1.0)) / NY as f32;
                let r = camera.get_ray(u, v);
                // let p = r.point_at_time(2.0);
                col += color(&r, &world);
            }
            col /= NS as f32;

            buffer.push((255.99 * col[0]) as u8);
            buffer.push((255.99 * col[1]) as u8);
            buffer.push((255.99 * col[2]) as u8);
        }
    }

    image::save_buffer("image.png", &buffer, NX, NY, image::RGB(8)).unwrap();
}
