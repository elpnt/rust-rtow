use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let target: Vec3 = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray {
            origin: rec.p,
            direction: target - rec.p,
        };
        let attenuation: Vec3 = self.albedo;

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let f: f32 = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflected: Vec3 = reflect(&r_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + f * random_in_unit_sphere(),
        };
        let attenuation: Vec3 = self.albedo;

        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::make_unit_vector();
    while p.squared_length() > 1.0 {
        p =
            2.0 * Vec3::new(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
            ) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(normal) * *normal
}
