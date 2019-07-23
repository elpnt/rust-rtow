use rand::prelude::*;
use std::fs;
use std::io::{BufWriter, Write};

mod camera;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hitable::Hitable;
use hitable_list::HitableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.0, std::f32::MAX) {
        0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0)
    } else {
        let unit_direction: Vec3 = r.direction.unit_vector();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn create_sphere(center: Vec3, radius: f32) -> Box<dyn Hitable + 'static> {
    Box::new(Sphere { center, radius })
}

fn main() {
    let nx: u32 = 800;
    let ny: u32 = 400;
    let ns: u32 = 100; // number of samples inside each pixel

    let mut f = BufWriter::new(fs::File::create("image/ch6-antialiasing.ppm").unwrap());
    f.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();

    let hitables = vec![
        create_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5),
        create_sphere(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];
    let world = HitableList { hitables };
    let cam: Camera = Default::default();

    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let ru: f32 = rng.gen();
                let rv: f32 = rng.gen();
                let u = (i as f32 + ru) / nx as f32;
                let v = (j as f32 + rv) / ny as f32;
                let r: Ray = cam.get_ray(u, v);
                // let p: Vec3 = r.point_at_parameter(2.0);
                col += color(&r, &world);
            }

            col /= ns as f32;
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            f.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}
