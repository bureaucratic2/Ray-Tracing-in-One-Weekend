use ray_tracing::{Color, Point3, Ray, Vec3};
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
    let lower_left_center = -origin - horizontal / 2 - vertical / 2 - Vec3::new(0, 0, focal_length);

    let mut f = File::create("image.ppm").unwrap();
    f.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .unwrap();

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(origin, lower_left_center + u * horizontal + v * vertical);
            let color = color(&ray);

            f.write_all(format!("{}", color).as_bytes()).unwrap();
        }
    }

    eprintln!("Done");

    println!("Hooray! This is the graphics \"hello world\".");
}

fn color(r: &Ray) -> Color {
    let unit_direction = r.direction();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // Color don't implement ops, so we should manipulate Vec3 first
    // and then wrap them as Color
    Color::from((1.0 - t) * Vec3::new(1, 1, 1) + t * Vec3::new(0.5, 0.7, 1))
}
