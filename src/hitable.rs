use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> bool {
        unimplemented!();
    }
}
