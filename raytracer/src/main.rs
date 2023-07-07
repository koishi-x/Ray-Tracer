mod camera;
mod color;
mod hittable_list;
mod material;
mod sphere;

use camera::*;
use color::*;
use hittable_list::*;
use material::*;
use raytracer::*;
use sphere::*;

fn ray_color(r: Ray, world: &impl Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new();
    }

    match world.hit(r, 0.001, INFINITY) {
        Some(rec) => match rec.mat_ptr.scatter(r, &rec) {
            None => Vec3::new(),
            Some((attenuation, scattered)) => attenuation * ray_color(scattered, world, depth - 1),
        },
        None => {
            let unit_direction = unit_vector(r.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            } * (1.0 - t)
                + Vec3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                } * t
        }
    }
}

fn main() {
    //path
    let path = std::path::Path::new("output/book1/image15.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

    //World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Vec3 {
        x: 0.8,
        y: 0.8,
        z: 0.0,
    }));

    //let material_center = Arc::new(Dielectric::new(1.5));
    let material_left = Arc::new(Dielectric::new(1.5));

    let material_center = Arc::new(Lambertian::new(Vec3 {
        x: 0.1,
        y: 0.2,
        z: 0.5,
    }));
    // let material_left = Arc::new(Metal::new(
    //     Vec3 {
    //         x: 0.8,
    //         y: 0.8,
    //         z: 0.8,
    //     },
    //     0.3,
    // ));
    let material_right = Arc::new(Metal::new(
        Vec3 {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        1.0,
    ));

    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        mat_ptr: material_ground,
    }));
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_ptr: material_center,
    }));
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_ptr: material_left,
    }));
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_ptr: material_right,
    }));

    //Camera
    let cam = Camera::new();

    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_width * image_height) as u64)
    };

    //Render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, image_height - j - 1);
            let mut pixel_color = Vec3::new();
            //println!("{i}, {j}, {s}");
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double_default()) / ((image_width - 1) as f64);
                let v = (j as f64 + random_double_default()) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            *pixel = image::Rgb(write_color(pixel_color, samples_per_pixel));

            progress.inc(1);
        }
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
