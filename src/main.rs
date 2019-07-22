use std::fs;
use std::io::{BufWriter, Write};

mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn color(r: Ray) -> Vec3 {
    let unit_direction = (r.direction).unit_vector();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    // 以下は the trait `std::ops::Mul<vec3::Vec3>` is not implemented for `f32`
    // としてコンパイルエラーになってしまう
    // (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(1.0, 1.0, 1.0)
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;

    let mut f = BufWriter::new(fs::File::create("image/ch3-ray.ppm").unwrap());
    f.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            /*
            let r = Ray {
                origin: origin,
                direction: lower_left_corner + horizontal * u + vertical * v,
            };
            */

            let col = color(r);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            f.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}
