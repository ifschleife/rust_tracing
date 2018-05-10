extern crate rand;
use cgmath::*;
use rand::Rng;

use ray::{Ray};


pub enum Material {
    Lambertian { albedo: Vector3<f32> },
    Metal { albedo: Vector3<f32>, fuzz: f32 },
}

pub struct ScatterRay {
    pub ray: Ray,
    pub attenuation: Vector3<f32>,
}

trait VectorLength {
    fn length(&self) -> f32;
    fn squared_length(&self) -> f32;
}

impl VectorLength for Vector3<f32> {
    fn length(&self) -> f32 {
        return self.squared_length().sqrt();
    }
    fn squared_length(&self) -> f32 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }
}

fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0f32*vec3(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)) - vec3(1.0, 1.0, 1.0);
        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_point: &Vector3<f32>, hit_normal: &Vector3<f32>) -> Option<ScatterRay> {
        match self {
            &Material::Lambertian {albedo } => return Material::scatter_lambertian(&hit_point, &hit_normal, &albedo),
            &Material::Metal { albedo, fuzz } => return Material::scatter_metal(&ray_in, &hit_point, &hit_normal, &albedo, fuzz),
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

    fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        return v - 2.0*v.dot(*n)*n;
    }
}
