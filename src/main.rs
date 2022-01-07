use ray_tracing::{Color, Point3, Vec3};
use std::{fs::File, io::Write};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400u64;
    let image_height = (400 as f64 / aspect_ratio) as u64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0, 0);
    let vertical = Vec3::new(0, viewport_height, 0);
    let lower_left_center =
        -origin - &horizontal / 2 - &vertical / 2 - Vec3::new(0, 0, focal_length);

    let mut f = File::create("image.ppm").unwrap();
    f.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .unwrap();

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            f.write_all(format!("{}", color).as_bytes()).unwrap();
        }
    }

    eprintln!("Done");

    println!("Hooray! This is the graphics \"hello world\".");
}
