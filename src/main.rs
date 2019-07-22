use std::fs;
use std::io::{BufWriter, Write};

mod vec3;

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;

    let mut f = BufWriter::new(fs::File::create("image/ch2-vec.ppm").unwrap());
    f.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = vec3::Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2f32);

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            f.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
                .unwrap();
        }
    }
}
