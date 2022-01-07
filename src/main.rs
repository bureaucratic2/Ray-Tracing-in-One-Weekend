use ray_tracing::Color;
use std::{fs::File, io::Write};

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    let mut f = File::create("image.ppm").unwrap();
    f.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .unwrap();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );

            f.write_all(format!("{}", color).as_bytes()).unwrap();
        }
    }

    eprintln!("Done");

    println!("Hooray! This is the graphics \"hello world\".");
}
