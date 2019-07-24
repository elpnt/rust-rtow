use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::borrow::Borrow;
use std::sync::Arc;

// #[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a: f32 = r.direction.dot(&r.direction);
        let b: f32 = oc.dot(&r.direction);
        let c: f32 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;

        if discriminant > 0. {
            // ray crosses the sphere at least once
            let temp1: f32 = (-b - discriminant.sqrt()) / a;
            let temp2: f32 = (-b + discriminant.sqrt()) / a;
            let b1: bool = t_max > temp1 && temp1 > t_min;
            let b2: bool = t_max > temp2 && temp2 > t_min;

            if discriminant > 0. && (b1 || b2) {
                // temp1の方がスクリーンに近い
                let t: f32 = if b1 { temp1 } else { temp2 };
                let p: Vec3 = r.point_at_parameter(t);
                let normal: Vec3 = (p - self.center) / self.radius;
                Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: self.material.borrow(),
                })
            } else {
                None
            }
        } else {
            // ray never crosses the sphere
            None
        }
    }
}
