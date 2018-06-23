extern crate image;
extern crate rand;
use rand::{Rng, SeedableRng, StdRng};
use std::f32;
use std::time::SystemTime;

mod camera;
mod hitable;
mod material;
mod math;
use camera::{Camera};
use hitable::{Hitable, World};
use material::{Material};
use math::*;
use std::env;

static mut COUNTER : u32 = 0;

fn color(ray: &Ray, world: &World, depth: i32, rng: &mut Rng) -> Vec3f {
    unsafe {
    COUNTER += 1;
    }
    match world.hit(&ray, 0.001, f32::MAX) {
        Some(record) => {
            if depth < 50 {
                match record.material.scatter(&ray, record.p, record.normal, rng) {
                    Some(scatter) => {
                        return scatter.attenuation * color(&scatter.ray, &world, depth+1, rng);
                    },
                    None => return Vec3f::zero(),
                }
            }
            else {
                return Vec3f::zero();
            }
        },
        None => {
            let unit_direction = normalize(ray.direction);
            let t = 0.5*(unit_direction.y+1.0);
            return (1.0-t)*Vec3f::one() + t*Vec3f::new(0.5, 0.7, 1.0);
        }
    }
}

fn random_scene(rng: &mut Rng) -> Vec<Hitable> {
    let n = 500;
    let mut objects = Vec::with_capacity(n);
    objects.push(Hitable::Sphere{center: vec3f(0.0, -1000.0, 0.0), radius: 1000.0, material: Material::Lambertian{albedo: vec3f(0.5, 0.5, 0.5)}});

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.next_f32();
            let center = vec3f(a as f32 + 0.9*rng.next_f32(), 0.2, b as f32 + 0.9*rng.next_f32());
            if (center-vec3f(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo_x = rng.next_f32() * rng.next_f32();
                    let albedo_y = rng.next_f32() * rng.next_f32();
                    let albedo_z = rng.next_f32() * rng.next_f32();
                    objects.push(Hitable::Sphere{center: center, radius: 0.2, material: Material::Lambertian{albedo: vec3f(albedo_x, albedo_y, albedo_z)}});
                }
                else if choose_mat < 0.95 {
                    let albedo_x = 0.5*(1.0 + rng.next_f32());
                    let albedo_y = 0.5*(1.0 + rng.next_f32());
                    let albedo_z = 0.5*(1.0 + rng.next_f32());
                    let fuzziness = 0.5*rng.next_f32();
                    objects.push(Hitable::Sphere{center: center, radius: 0.2, material: Material::Metal{albedo: vec3f(albedo_x, albedo_y, albedo_z), fuzz: fuzziness}});
                }
                else {
                    objects.push(Hitable::Sphere{center: center, radius: 0.2, material: Material::Dielectric{refraction_index: 1.5}});
                }
            }
        }
    }

    objects.push(Hitable::Sphere{center: vec3f(0.0, 1.0, 0.0), radius: 1.0, material: Material::Dielectric{refraction_index: 1.5}});
    objects.push(Hitable::Sphere{center: vec3f(-4.0, 1.0, 0.0), radius: 1.0, material: Material::Lambertian{albedo: vec3f(0.4, 0.2, 0.1)}});
    objects.push(Hitable::Sphere{center: vec3f(4.0, 1.0, 0.0), radius: 1.0, material: Material::Metal{albedo: vec3f(0.7, 0.6, 0.5), fuzz: 0.0}});
    return objects;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please specify width and height of rendered image", );
        return;
    }
    let width: u32 = args[1].parse().unwrap();
    let height: u32 = args[2].parse().unwrap();
    const SAMPLE_COUNT: u32 = 10;

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng : StdRng =  SeedableRng::from_seed(seed);

    let world = World{objects: random_scene(&mut rng)};

    let look_from = vec3f(13.0, 2.0, 3.0);
    let look_at = Vec3f::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let aspect = width as f32 / height as f32;
    let camera = Camera::new(look_from, look_at, vec3f(0.0, 1.0, 0.0), 20.0, aspect, aperture, dist_to_focus);

    let buffer_size: usize = (width*height*3) as usize;
    let mut buffer = Vec::with_capacity(buffer_size);
    let start_time = SystemTime::now();

    for y in (0..height).rev() {
        for x in 0..width {
            let mut col = Vec3f::zero();
            for _ in 0..SAMPLE_COUNT {
                let u = (x as f32 + rng.next_f32()) / width as f32;
                let v = (y as f32 + rng.next_f32()) / height as f32;
                let r = camera.get_ray(u, v, &mut rng);
                col += color(&r, &world, 0, &mut rng);
            }
            col /= SAMPLE_COUNT as f32;
            col = vec3f(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            buffer.push((255.99 * col.x) as u8);
            buffer.push((255.99 * col.y) as u8);
            buffer.push((255.99 * col.z) as u8);
        }
    }

    let elapsed = start_time.elapsed().expect("SystemTime elapsed time failed");
    unsafe {
        println!("{}.{} seconds\n{} rays", elapsed.as_secs(), elapsed.subsec_millis(), COUNTER);
    }
    image::save_buffer("output.png", &buffer, width, height, image::RGB(8)).unwrap();
}
