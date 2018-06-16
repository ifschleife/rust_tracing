extern crate cgmath;
extern crate image;
extern crate rand;
use cgmath::*;
use rand::Rng;
use std::f32;
use std::time::SystemTime;

mod camera;
mod hitable;
mod material;
mod ray;
mod vector;
use camera::{Camera};
use hitable::{Hitable, World};
use material::{Material};
use ray::{Ray};
use vector::{VectorLength};

static mut COUNTER : u32 = 0;

fn color(ray: &Ray, world: &World, depth: i32) -> Vector3<f32> {
    unsafe {
    COUNTER += 1;
    }
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

fn random_scene() -> Vec<Hitable> {
    let n = 500;
    let mut objects = Vec::with_capacity(n);
    objects.push(Hitable::Sphere{center: vec3(0.0, -1000.0, 0.0), radius: 1000.0, material: Material::Lambertian{albedo: vec3(0.5, 0.5, 0.5)}});

    let mut rng = rand::weak_rng();//thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.next_f32();
            let center = vec3(a as f32 + 0.9*rng.next_f32(), 0.2, b as f32 + 0.9*rng.next_f32());
            if (center-vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo_x = rng.next_f32() * rng.next_f32();
                    let albedo_y = rng.next_f32() * rng.next_f32();
                    let albedo_z = rng.next_f32() * rng.next_f32();
                    objects.push(Hitable::Sphere{center: center, radius: 0.2, material: Material::Lambertian{albedo: vec3(albedo_x, albedo_y, albedo_z)}});
                }
                else if choose_mat < 0.95 {
                    let albedo_x = 0.5*(1.0 + rng.next_f32());
                    let albedo_y = 0.5*(1.0 + rng.next_f32());
                    let albedo_z = 0.5*(1.0 + rng.next_f32());
                    let fuzziness = 0.5*rng.next_f32();
                    objects.push(Hitable::Sphere{center: center, radius: 0.2, material: Material::Metal{albedo: vec3(albedo_x, albedo_y, albedo_z), fuzz: fuzziness}});
                }
                else {
                    objects.push(Hitable::Sphere{center: center, radius: 0.2, material: Material::Dielectric{refraction_index: 1.5}});
                }
            }
        }
    }

    objects.push(Hitable::Sphere{center: vec3(0.0, 1.0, 0.0), radius: 1.0, material: Material::Dielectric{refraction_index: 1.5}});
    objects.push(Hitable::Sphere{center: vec3(-4.0, 1.0, 0.0), radius: 1.0, material: Material::Lambertian{albedo: vec3(0.4, 0.2, 0.1)}});
    objects.push(Hitable::Sphere{center: vec3(4.0, 1.0, 0.0), radius: 1.0, material: Material::Metal{albedo: vec3(0.7, 0.6, 0.5), fuzz: 0.0}});
    return objects;
}

fn main() {
    const NX: u32 = 600;
    const NY: u32 = 400;
    const NS: u32 = 10;
    const BUFFER_SIZE: usize = (NX*NY*3) as usize;

    let world = World{objects: random_scene()};

    let look_from = vec3(13.0, 2.0, 3.0);
    let look_at = vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let aspect = NX as f32 / NY as f32;
    let camera = Camera::new(look_from, look_at, vec3(0.0, 1.0, 0.0), 20.0, aspect, aperture, dist_to_focus);
    let mut rng = rand::weak_rng();

    let mut buffer = Vec::with_capacity(BUFFER_SIZE);
    let start_time = SystemTime::now();

    for y in (0..NY).rev() {
        for x in 0..NX {
            let mut col = vec3(0.0, 0.0, 0.0);
            for _ in 0..NS {
                let u = (x as f32 + rng.next_f32()) / NX as f32;
                let v = (y as f32 + rng.next_f32()) / NY as f32;
                let r = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= NS as f32;
            col = vec3(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            buffer.push((255.99 * col[0]) as u8);
            buffer.push((255.99 * col[1]) as u8);
            buffer.push((255.99 * col[2]) as u8);
        }
    }

    let elapsed = start_time.elapsed().expect("SystemTime elapsed time failed");
    unsafe {
        let milli = (elapsed.subsec_nanos() * 1_000_000_000).to_string();
        let milli = &milli[0..2];
        println!("{}.{} seconds\n{} rays", elapsed.as_secs(), milli, COUNTER);
    }
    image::save_buffer("output.png", &buffer, NX, NY, image::RGB(8)).unwrap();
}
