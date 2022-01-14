use rand::prelude::*;
use ray_tracing::{
    Camera, Color, Dielectritic, HitRecord, Hittable, HittableList, Lambertian, Material, Metal,
    Point3, Ray, Sphere, Vec3,
};
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
};

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material: Arc<Box<dyn Material>> =
        Arc::new(Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))));
    world.add(Arc::new(Sphere::new(
        Point3::new(0, -1000, 0),
        1000.0,
        Arc::clone(&ground_material),
    )));

    let fixed_p = Point3::new(4, 0.2, 0);
    let mut rng = StdRng::from_rng(thread_rng()).unwrap();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - fixed_p).length() > 0.9 {
                // walkaround Rust type inference
                // reference: https://stackoverflow.com/questions/61972343/why-cant-i-push-into-a-vec-of-dyn-trait-unless-i-use-a-temporary-variable
                let sphere_material: Arc<Box<dyn Material>> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        Color::from(Vec3::random(&mut rng, 0, 1) * Vec3::random(&mut rng, 0, 1));
                    Arc::new(Box::new(Lambertian::new(albedo)))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::from(Vec3::random(&mut rng, 0.5, 1));
                    let fuzz = rng.gen_range(0.0..1.0);
                    Arc::new(Box::new(Metal::new(albedo, fuzz)))
                } else {
                    // glass
                    Arc::new(Box::new(Dielectritic::new(1.5)))
                };

                world.add(Arc::new(Sphere::new(
                    center,
                    0.2,
                    Arc::clone(&sphere_material),
                )));
            }
        }
    }

    let material1: Arc<Box<dyn Material>> = Arc::new(Box::new(Dielectritic::new(1.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0, 1, 0),
        1.0,
        Arc::clone(&material1),
    )));

    let material2: Arc<Box<dyn Material>> =
        Arc::new(Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4, 1, 0),
        1.0,
        Arc::clone(&material2),
    )));

    let material3: Arc<Box<dyn Material>> =
        Arc::new(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Arc::new(Sphere::new(
        Point3::new(4, 1, 0),
        1.0,
        Arc::clone(&material3),
    )));

    world
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // Camera
    let look_from = Point3::new(13, 2, 3);
    let look_at = Point3::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut f = BufWriter::new(File::create("image.ppm").unwrap());
    f.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .unwrap();

    // World
    let world = random_scene();

    // Render
    let image = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            eprintln!("Scanline: {}", j);
            let mut rng = StdRng::from_rng(thread_rng()).unwrap();
            let mut line = vec![];
            for i in 0..image_width {
                let mut pixel_color = Vec3::default();
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                    let ray = camera.get_ray(&mut rng, u, v);
                    pixel_color += *ray_color(&ray, &world, max_depth);
                }
                line.push(Color::from(pixel_color));
            }
            line
        })
        .collect::<Vec<_>>();

    for line in image {
        for c in line {
            write_color(&mut f, c, samples_per_pixel);
        }
    }

    f.flush().unwrap();
    eprintln!("Done");
}

fn ray_color(r: &Ray, world: &HittableList, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0, 0, 0);
    }

    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        if let Some(ref material) = rec.material {
            if let Some((attenuation, scattered)) = material.scatter(r, &rec) {
                return Color::from(*attenuation * *ray_color(&scattered, world, depth - 1));
            }
        }
        return Color::new(0, 0, 0);
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
