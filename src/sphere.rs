use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
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
                Some(HitRecord { t, p, normal })
            } else {
                None
            }
        /* too redundant!!
        let mut temp: f32 = (-b - discriminant.sqrt()) / a;
        if temp < t_max && temp > t_min {
            let t = temp;
            let p = r.point_at_parameter(t);
            let normal = (p - self.center) / self.radius;
            Some(HitRecord { t, p, normal })
        } else {
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                Some(HitRecord { t, p, normal })
            } else {
                None
            }
        }
        */
        } else {
            // ray never crosses the sphere
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit() {
        let sp = Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let r = Ray {
            origin: Vec3::new(0.0, 0.0, 10.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
        };
        let t_min: f32 = 0.0;
        let t_max: f32 = std::f32::MAX;

        // expected hitrecord's parameter derived analytically
        let t: f32 = 9.0;
        let p: Vec3 = Vec3::new(0.0, 0.0, 1.0);
        let normal: Vec3 = Vec3::new(0.0, 0.0, 1.0);

        let maybe_hit: Option<HitRecord> = sp.hit(&r, t_min, t_max);
        assert_eq!(maybe_hit.unwrap(), HitRecord { t, p, normal });
    }

    #[test]
    fn not_hit() {
        let sp = Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let r = Ray {
            origin: Vec3::new(0.0, 0.0, 10.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
        };
        let t_min: f32 = 0.0;
        let t_max: f32 = std::f32::MAX;

        let maybe_hit: Option<HitRecord> = sp.hit(&r, t_min, t_max);
        assert_eq!(maybe_hit.is_none(), true);
    }
}
