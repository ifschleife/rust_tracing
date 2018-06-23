use rand::prelude::*;

use math::*;


pub enum Material {
    Lambertian { albedo: Vec3f },
    Metal { albedo: Vec3f, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}

pub struct ScatterRay {
    pub ray: Ray,
    pub attenuation: Vec3f,
}

fn random_in_unit_sphere(rng: &mut SmallRng) -> Vec3f {
    loop {
        let p = 2.0f32*vec3f(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3f::one();
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_point: Vec3f, hit_normal: Vec3f, rng: &mut SmallRng) -> Option<ScatterRay> {
        match self {
            &Material::Lambertian {albedo } => return Material::scatter_lambertian(hit_point, hit_normal, albedo, rng),
            &Material::Metal { albedo, fuzz } => return Material::scatter_metal(&ray_in, hit_point, hit_normal, albedo, fuzz, rng),
            &Material::Dielectric { refraction_index } => return Material::scatter_dielectric(&ray_in, hit_point, hit_normal, refraction_index, rng),
        }
    }

    fn scatter_lambertian(hit_point: Vec3f, hit_normal: Vec3f, albedo: Vec3f, rng: &mut SmallRng) -> Option<ScatterRay> {
        let target = hit_point + hit_normal + random_in_unit_sphere(rng);
        Some(ScatterRay{ray: Ray::new(hit_point, target - hit_point), attenuation: albedo })
    }

    fn scatter_metal(ray_in: &Ray, hit_point: Vec3f, hit_normal: Vec3f, albedo: Vec3f, fuzz: f32, rng: &mut SmallRng) -> Option<ScatterRay> {
        let mut fuzziness = fuzz;
        if fuzz >= 1.0 {
            fuzziness = 1.0
        }
        let reflected = Material::reflect(normalize(ray_in.direction), hit_normal);
        let scattered = Ray::new(hit_point, reflected + fuzziness*random_in_unit_sphere(rng));
        if dot(scattered.direction, hit_normal) > 0.0 {
            return Some(ScatterRay{ray: scattered, attenuation: albedo});
        }
        None
    }

    fn scatter_dielectric(ray_in: &Ray, hit_point: Vec3f, hit_normal: Vec3f, ref_idx: f32, rng: &mut SmallRng) -> Option<ScatterRay> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let ray_hit_normal_angle = dot(ray_in.direction, hit_normal);

        if ray_hit_normal_angle > 0.0 {
            outward_normal = -hit_normal;
            ni_over_nt = ref_idx;
            cosine = ref_idx * ray_hit_normal_angle / ray_in.direction.length();
        }
        else {
            outward_normal = hit_normal;
            ni_over_nt = 1.0 / ref_idx;
            cosine = -ray_hit_normal_angle / ray_in.direction.length();
        }

        match Material::refract(ray_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let scattered;
                if rng.gen::<f32>() < Material::schlick(cosine, ref_idx) {
                    scattered = Ray::new(hit_point, Material::reflect(ray_in.direction, hit_normal));
                }
                else {
                    scattered = Ray::new(hit_point, refracted);
                }

                Some(ScatterRay{ray: scattered, attenuation: Vec3f::one()})
            },
            None => {
                let scattered = Ray::new(hit_point, Material::reflect(ray_in.direction, hit_normal));
                Some(ScatterRay{ray: scattered, attenuation: Vec3f::one()})
            }
        }
    }

    fn reflect(v: Vec3f, n: Vec3f) -> Vec3f {
        return v - 2.0 * dot(v, n) * n;
    }

    fn refract(vec: Vec3f, normal: Vec3f, ni_over_nt: f32) -> Option<Vec3f> {
        let uv = normalize(vec);
        let dt = dot(uv, normal);
        let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
        if discriminant > 0.0 {
            let refracted = ni_over_nt*(uv - normal * dt) - normal * discriminant.sqrt();
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
