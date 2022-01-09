use rand::Rng;
use ray_tracing::{
    random_unit_vector, Camera, Color, HitRecord, Hittable, HittableList, Point3, Ray, Sphere, Vec3,
};
use std::{fs::File, io::Write, rc::Rc};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400u64;
    let image_height = (400_f64 / aspect_ratio) as u64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let camera = Camera::new();

    // World
    let mut list = HittableList::new(Rc::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    list.add(Rc::new(Sphere::new(Point3::new(0, -100.5, -1), 100.0)));

    let mut f = File::create("image.ppm").unwrap();
    f.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .unwrap();

    // Render
    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += *ray_color(&ray, &list, max_depth);
            }
            let pixel_color = Color::from(pixel_color);

            // f.write_all(format!("{}", color).as_bytes()).unwrap();
            write_color(&mut f, pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done");

    println!("Hooray! This is the graphics \"hello world\".");
}

fn ray_color(r: &Ray, world: &HittableList, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0, 0, 0);
    }

    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        // // map each component(-1~1) to the interval from 0 to 1
        // rec.normal += Vec3::new(1, 1, 1);
        // rec.normal /= 2;
        // return Color::from(rec.normal);
        let target = rec.p + rec.normal + random_unit_vector();
        return Color::from(0.5 * *ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // Color don't implement ops, so we should manipulate Vec3 first
    // and then wrap them as Color
    Color::from((1.0 - t) * Vec3::new(1, 1, 1) + t * Vec3::new(0.5, 0.7, 1))
}

fn write_color<F: Write>(f: &mut F, mut pixel_color: Color, samples_per_pixel: usize) {
    let scale = 1.0 / (samples_per_pixel as f64);
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    for i in 0..3 {
        pixel_color[i] = (pixel_color[i] * scale).sqrt();
    }

    f.write_all(format!("{}", pixel_color).as_bytes()).unwrap();
}
