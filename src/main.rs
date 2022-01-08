use ray_tracing::{Color, Point3, Ray, Vec3};
use std::{fs::File, io::Write};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400u64;
    let image_height = (400_f64 / aspect_ratio) as u64;

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

    // Render
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
    let mut t = hit_sphere(&Point3::new(0, 0, -1), 0.5, r);
    if t > 0.0 {
        let mut n = (r.at(t) - Vec3::new(0, 0, -1)).unit_vector();
        // map each component(-1~1) to the interval from 0 to 1
        n += Vec3::new(1, 1, 1);
        n /= 2;
        return Color::from(n);
    }
    let unit_direction = r.direction().unit_vector();
    t = 0.5 * (unit_direction.y() + 1.0);
    // Color don't implement ops, so we should manipulate Vec3 first
    // and then wrap them as Color
    Color::from((1.0 - t) * Vec3::new(1, 1, 1) + t * Vec3::new(0.5, 0.7, 1))
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = r.direction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
        (-half_b - discriminant.sqrt()) / a
    } else {
        -1.0
    }
}
