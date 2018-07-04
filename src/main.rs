extern crate image;
extern crate rand;
extern crate rayon;
use rand::prelude::*;
use rayon::prelude::*;
use std::env;
use std::f32;
use std::time::SystemTime;

mod camera;
mod hitable;
mod material;
mod math;
mod scene;

use camera::{Camera};
use math::*;
use scene::{Scene};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please specify width and height of rendered image", );
        return;
    }
    let width: usize = args[1].parse().unwrap();
    let height: usize = args[2].parse().unwrap();
    const SAMPLE_COUNT: u32 = 10;

    let seed: [u8; 16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
    let mut rng = SmallRng::from_seed(seed);

    let scene = Scene::generate(&mut rng);

    let look_from = vec3f(13.0, 2.0, 3.0);
    let look_at = Vec3f::zero();
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let aspect = width as f32 / height as f32;
    let camera = Camera::new(look_from, look_at, vec3f(0.0, 1.0, 0.0), 20.0, aspect, aperture, dist_to_focus);

    const COLOR_DEPTH: usize = 3;
    let buffer_size = width * height * COLOR_DEPTH;
    let mut buffer = vec![0u8; buffer_size];
    let start_time = SystemTime::now();

    buffer.par_chunks_mut(COLOR_DEPTH * width).rev().enumerate().for_each(|(y, row)| {
        let mut rng = SmallRng::from_seed(seed);
        for (x, rgb) in row.chunks_mut(3).enumerate() {
            let mut col = Vec3f::zero();
            for _ in 0..SAMPLE_COUNT {
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = (y as f32 + rng.gen::<f32>()) / height as f32;
                let r = camera.get_ray(u, v, &mut rng);
                col += scene.ray_trace(&r, 0, &mut rng);
            }
            col /= SAMPLE_COUNT as f32;
            col = vec3f(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            rgb[0] = (255.99 * col.x) as u8;
            rgb[1] = (255.99 * col.y) as u8;
            rgb[2] = (255.99 * col.z) as u8;
        }
    });

    let elapsed = start_time.elapsed().expect("SystemTime elapsed time failed");
    println!("{}.{} seconds\n{} rays", elapsed.as_secs(), elapsed.subsec_millis(), scene.get_ray_count());

    image::save_buffer("output.png", &buffer, width as u32, height as u32, image::RGB(8)).unwrap();
}
