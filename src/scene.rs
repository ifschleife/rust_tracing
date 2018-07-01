use rand::prelude::*;
use std::f32;

use hitable::{Sphere};
use math::*;
use material::{Material, ScatterRay};


pub struct Scene {
    objects: Vec<Sphere>,
    pub ray_count: u32,
}

impl Scene {
    pub fn generate(rng: &mut SmallRng) -> Scene {
        let n = 500;
        let mut scene = Scene { objects: Vec::with_capacity(n), ray_count: 0 };
        scene.objects.push(Sphere{center: vec3f(0.0, -1000.0, 0.0), radius: 1000.0, material: Material::Lambertian{albedo: vec3f(0.5, 0.5, 0.5)}});

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rng.gen::<f32>();
                let center = vec3f(a as f32 + 0.9*rng.gen::<f32>(), 0.2, b as f32 + 0.9*rng.gen::<f32>());
                if (center-vec3f(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo_x = rng.gen::<f32>() * rng.gen::<f32>();
                        let albedo_y = rng.gen::<f32>() * rng.gen::<f32>();
                        let albedo_z = rng.gen::<f32>() * rng.gen::<f32>();
                        scene.objects.push(Sphere{center: center, radius: 0.2, material: Material::Lambertian{albedo: vec3f(albedo_x, albedo_y, albedo_z)}});
                    }
                    else if choose_mat < 0.95 {
                        let albedo_x = 0.5*(1.0 + rng.gen::<f32>());
                        let albedo_y = 0.5*(1.0 + rng.gen::<f32>());
                        let albedo_z = 0.5*(1.0 + rng.gen::<f32>());
                        let fuzziness = 0.5*rng.gen::<f32>();
                        scene.objects.push(Sphere{center: center, radius: 0.2, material: Material::Metal{albedo: vec3f(albedo_x, albedo_y, albedo_z), fuzz: fuzziness}});
                    }
                    else {
                        scene.objects.push(Sphere{center: center, radius: 0.2, material: Material::Dielectric{refraction_index: 1.5}});
                    }
                }
            }
        }

        scene.objects.push(Sphere{center: vec3f(0.0, 1.0, 0.0), radius: 1.0, material: Material::Dielectric{refraction_index: 1.5}});
        scene.objects.push(Sphere{center: vec3f(-4.0, 1.0, 0.0), radius: 1.0, material: Material::Lambertian{albedo: vec3f(0.4, 0.2, 0.1)}});
        scene.objects.push(Sphere{center: vec3f(4.0, 1.0, 0.0), radius: 1.0, material: Material::Metal{albedo: vec3f(0.7, 0.6, 0.5), fuzz: 0.0}});
        return scene;
    }

    pub fn ray_trace(&mut self, ray: &Ray, depth: i32, rng: &mut SmallRng) -> Vec3f {
        self.ray_count += 1;

        if depth >= 50 {
            return Vec3f::zero();
        }

        if let Some(scatter) = self.hit(ray, 0.001, f32::MAX, rng) {
            return scatter.attenuation * self.ray_trace(&scatter.ray, depth+1, rng);
        } else {
            let unit_direction = normalize(ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * vec3f(1.0, 1.0, 1.0) + (t * Vec3f::new(0.5, 0.7, 1.0));
        }
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rng: &mut SmallRng) -> Option<ScatterRay> {
        let mut closest_so_far = t_max;
        let mut hit_object: Option<&Sphere> = None;

        for object in &self.objects {
            if let Some(ray_param) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = ray_param;
                hit_object = Some(object);
            }
        }

        if let Some(&object) = hit_object {
            let point = ray.point_at_time(closest_so_far);
            let normal = (point - object.center) / object.radius;

            return object.material.scatter(&ray, point, normal, rng);
        }
        
        None
    }
}
