use std::fs;
use std::io::{BufWriter, Write};

fn main() {
    let mut f = BufWriter::new(fs::File::create("image/writesample.ppm").unwrap());
    for _ in 0..100 {
        f.write(b"hoge").unwrap();
        f.write(b"\n").unwrap();
    }
}
