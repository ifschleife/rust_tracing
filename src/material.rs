extern crate rand;
use cgmath::*;
use rand::Rng;
use std::f32;

use ray::{Ray};
use vector::{VectorLength};


pub enum Material {
    Lambertian { albedo: Vector3<f32> },
    Metal { albedo: Vector3<f32>, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}

pub struct ScatterRay {
    pub ray: Ray,
    pub attenuation: Vector3<f32>,
}

fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::weak_rng();

    loop {
        let p = 2.0f32*vec3(rng.next_f32(), rng.next_f32(), rng.next_f32()) - vec3(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_point: &Vector3<f32>, hit_normal: &Vector3<f32>) -> Option<ScatterRay> {
        match self {
            &Material::Lambertian {albedo } => return Material::scatter_lambertian(&hit_point, &hit_normal, &albedo),
            &Material::Metal { albedo, fuzz } => return Material::scatter_metal(&ray_in, &hit_point, &hit_normal, &albedo, fuzz),
            &Material::Dielectric { refraction_index } => return Material::scatter_dielectric(&ray_in, &hit_point, &hit_normal, refraction_index),
        }
    }

    fn scatter_lambertian(hit_point: &Vector3<f32>, hit_normal: &Vector3<f32>, albedo: &Vector3<f32>) -> Option<ScatterRay> {
        let target = hit_point + hit_normal + random_in_unit_sphere();
        Some(ScatterRay{ray: Ray::new(*hit_point, target-hit_point), attenuation: *albedo })
    }

    fn scatter_metal(ray_in: &Ray, hit_point: &Vector3<f32>, hit_normal: &Vector3<f32>, albedo: &Vector3<f32>, fuzz: f32) -> Option<ScatterRay> {
        let mut fuzziness = fuzz;
        if fuzz >= 1.0 {
            fuzziness = 1.0
        }
        let reflected = Material::reflect(&ray_in.direction.normalize(), &hit_normal);
        let scattered = Ray::new(*hit_point, reflected + fuzziness*random_in_unit_sphere());
        if scattered.direction.dot(*hit_normal) > 0.0 {
            return Some(ScatterRay{ray: scattered, attenuation: *albedo});
        }
        None
    }

    fn scatter_dielectric(ray_in: &Ray, hit_point: &Vector3<f32>, hit_normal: &Vector3<f32>, ref_idx: f32) -> Option<ScatterRay> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let ray_hit_normal_angle = ray_in.direction.dot(*hit_normal);

        if ray_hit_normal_angle > 0.0 {
            outward_normal = -(*hit_normal);
            ni_over_nt = ref_idx;
            cosine = ref_idx * ray_hit_normal_angle / ray_in.direction.length();
        }
        else {
            outward_normal = *hit_normal;
            ni_over_nt = 1.0 / ref_idx;
            cosine = -ray_hit_normal_angle / ray_in.direction.length();
        }

        let mut rng = rand::weak_rng();
        let rand_value = rng.next_f32();

        match Material::refract(&ray_in.direction, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                let scattered;
                if rand_value < Material::schlick(cosine, ref_idx) {
                    scattered = Ray::new(*hit_point, Material::reflect(&ray_in.direction, &hit_normal));
                }
                else {
                    scattered = Ray::new(*hit_point, refracted);
                }

                Some(ScatterRay{ray: scattered, attenuation: vec3(1.0, 1.0, 1.0)})
            },
            None => {
                let scattered = Ray::new(*hit_point, Material::reflect(&ray_in.direction, &hit_normal));
                Some(ScatterRay{ray: scattered, attenuation: vec3(1.0, 1.0, 1.0)})
            }
        }
    }

    fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        return v - 2.0*v.dot(*n)*n;
    }

    fn refract(vec: &Vector3<f32>, normal: &Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
        let uv = vec.normalize();
        let dt = uv.dot(*normal);
        let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
        if discriminant > 0.0 {
            let refracted = ni_over_nt*(uv - normal*dt) - normal*discriminant.sqrt();
            return Some(refracted);
        }
        None
    }

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0-ref_idx) / (1.0+ref_idx);
        let r0 = r0*r0;
        r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
    }
}
