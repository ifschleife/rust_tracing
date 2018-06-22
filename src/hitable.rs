use material::{Material};
use math::*;


pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3f,
    pub normal: Vec3f,
    pub material: &'a Material,
}

pub enum Hitable {
    Sphere { center: Vec3f, radius: f32, material: Material },
    // Cube { center: Vec3f, half_size: f32 },
}

impl Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            &Hitable::Sphere { ref center, radius, ref material } => {
                return Hitable::hit_sphere(&center, radius, &material, ray, t_min, t_max);
            },
            // &Hitable::Cube { .. } => (),
        }
    }

    fn hit_sphere<'a>(center: &Vec3f, radius: f32, material: &'a Material, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>> {
        let oc = ray.origin - (*center);
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - radius * radius;
        let discriminant = (b * b) - (a * c);
        if discriminant > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at_time(temp);
                let normal = (point - *center) / radius;
                return Some(HitRecord{t : temp, p: point, normal: normal, material: &material});
            }
            let temp = (-b + (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at_time(temp);
                let normal = (point - *center) / radius;
                return Some(HitRecord{t: temp, p: point, normal: normal, material: &material});
            }
        }
        None
    }
}

pub struct World {
    pub objects: Vec<Hitable>
}

impl World {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_rec: Option<HitRecord> = None;
        for ref object in self.objects.iter() {
            match object.hit(ray, t_min, closest_so_far) {
                Some(record) => {
                    closest_so_far = record.t;
                    hit_rec = Some(record);
                },
                None => ()
            }
        }

        hit_rec
    }
}
