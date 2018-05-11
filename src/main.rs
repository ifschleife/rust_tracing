extern crate cgmath;
extern crate image;
extern crate rand;
use cgmath::*;
use rand::Rng;
use std::f32;

mod camera;
mod hitable;
mod material;
mod ray;
use camera::{Camera};
use hitable::{Hitable, World};
use material::{Material};
use ray::{Ray};


fn color(ray: &Ray, world: &World, depth: i32) -> Vector3<f32> {
    match world.hit(&ray, 0.001, f32::MAX) {
        Some(record) => {
            if depth < 50 {
                match record.material.scatter(&ray, &record.p, &record.normal) {
                    Some(scatter) => {
                        return scatter.attenuation.mul_element_wise(color(&scatter.ray, &world, depth+1));
                    },
                    None => return vec3(0.0, 0.0, 0.0),
                }
            }
            else {
                return vec3(0.0, 0.0, 0.0);
            }
        },
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
    world.objects.push(Hitable::Sphere{center: vec3(0.0, 0.0, -1.0), radius: 0.5, material: Material::Lambertian{albedo: vec3(0.1, 0.2, 0.5)}});
    world.objects.push(Hitable::Sphere{center: vec3(0.0, -100.5, -1.0), radius: 100.0, material: Material::Lambertian{albedo: vec3(0.8, 0.8, 0.0)}});
    world.objects.push(Hitable::Sphere{center: vec3(1.0, 0.0, -1.0), radius: 0.5, material: Material::Metal{albedo: vec3(0.8, 0.6, 0.2), fuzz: 0.2}});
    world.objects.push(Hitable::Sphere{center: vec3(-1.0, 0.0, -1.0), radius: 0.5, material: Material::Dielectric{refraction_index: 1.5}});
    world.objects.push(Hitable::Sphere{center: vec3(-1.0, 0.0, -1.0), radius: -0.45, material: Material::Dielectric{refraction_index: 1.5}});

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
                col += color(&r, &world, 0);
            }
            col /= NS as f32;
            col = vec3(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            buffer.push((255.99 * col[0]) as u8);
            buffer.push((255.99 * col[1]) as u8);
            buffer.push((255.99 * col[2]) as u8);
        }
    }

    image::save_buffer("image.png", &buffer, NX, NY, image::RGB(8)).unwrap();
}
