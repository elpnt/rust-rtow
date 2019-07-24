use rand;
use std::f32::consts::PI;
use std::fs;
use std::io::{BufWriter, Write};
use std::sync::Arc;

mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::Hitable;
use hitable_list::HitableList;
use material::*;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(r: &Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        if let Some(scatter_record) = rec.material.scatter(&r, &rec) {
            if depth < 50 {
                let attenuation: Vec3 = scatter_record.attenuation;
                let scattered: Ray = scatter_record.scattered;
                attenuation * color(&scattered, &world, depth + 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction: Vec3 = r.direction.unit_vector();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx: u32 = 800;
    let ny: u32 = 400;
    let ns: u32 = 50; // number of samples inside each pixel

    let mut f = BufWriter::new(fs::File::create("image/ch9-camera2.ppm").unwrap());
    f.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();

    let hitables = vec![
        Arc::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Arc::new(Lambertian {
                albedo: Vec3::new(0.1, 0.2, 0.5),
            }),
        }),
        Arc::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Arc::new(Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            }),
        }),
        Arc::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Arc::new(Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                fuzz: 0.3,
            }),
        }),
        Arc::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Arc::new(Dielectric { refract_idx: 1.5 }),
        }),
        Arc::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Arc::new(Dielectric { refract_idx: 1.5 }),
        }),
    ];
    let world = HitableList { hitables };
    let cam = Camera::new(
        Vec3::new(-1.0, 1.0, 0.5),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        nx as f32 / ny as f32,
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                let r: Ray = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }

            col /= ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            f.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}
