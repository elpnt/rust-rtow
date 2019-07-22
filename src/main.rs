use std::fs;
use std::io::{BufWriter, Write};

mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> bool {
    let oc: Vec3 = r.origin - center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(r: Ray) -> Vec3 {
    if hit_sphere(vec3![0.0, 0.0, -1.0], 0.5, r) {
        vec3![1.0, 0.0, 0.0]
    } else {
        let unit_direction = (r.direction).unit_vector();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * vec3![1.0, 1.0, 1.0] + t * vec3![0.5, 0.7, 1.0]
    }
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;

    let mut f = BufWriter::new(fs::File::create("image/ch4-sphere.ppm").unwrap());
    f.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();

    let lower_left_corner = vec3![-2.0, -1.0, -1.0];
    let horizontal = vec3![4.0, 0.0, 0.0];
    let vertical = vec3![0.0, 2.0, 0.0];
    let origin = vec3![0.0, 0.0, 0.0];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col = color(r);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            f.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}
