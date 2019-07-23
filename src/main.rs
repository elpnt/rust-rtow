use std::fs;
use std::io::{BufWriter, Write};

mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc: Vec3 = r.origin - center;
    let a: f32 = r.direction.dot(&r.direction);
    let b: f32 = 2.0 * oc.dot(&r.direction);
    let c: f32 = oc.dot(&oc) - radius * radius;
    let discriminant: f32 = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(r: Ray) -> Vec3 {
    let t: f32 = hit_sphere(vec3![0.0, 0.0, -1.0], 0.5, r);
    if t > 0.0 {
        let normal: Vec3 = (r.point_at_parameter(t) - vec3![0.0, 0.0, -1.0]).unit_vector();
        0.5 * vec3![normal.x + 1., normal.y + 1., normal.z + 1.]
    } else {
        let unit_direction = (r.direction).unit_vector();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * vec3![1.0, 1.0, 1.0] + t * vec3![0.5, 0.7, 1.0]
    }
}

fn main() {
    let nx: u32 = 400;
    let ny: u32 = 200;

    let mut f = BufWriter::new(fs::File::create("image/ch5-sphere.ppm").unwrap());
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
