mod camera;
mod color;
mod hittable_list;
mod sphere;

use camera::*;
use color::*;
use hittable_list::*;
use raytracer::*;
use sphere::*;
//use std::sync::Arc;

fn ray_color(r: &Ray, world: &impl Hittable) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal
            + Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            })
            * 0.5;
    }
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

    /*
    let center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let tmp = Sphere {
        center,
        radius: 0.5,
    };
    let mut hit_record = HitRecord::new();
    if tmp.hit(&r, 0.0, 114514.0, &mut hit_record) {
        return Vec3 {
            x: hit_record.normal.x + 1.0,
            y: hit_record.normal.y + 1.0,
            z: hit_record.normal.z + 1.0,
        } * 0.5;
    }
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
        */
}
fn main() {
    //path
    let path = std::path::Path::new("output/book1/image6.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;

    //World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));

    //Camera
    let cam = Camera::new();
    // let viewport_height = 2.0;
    // let viewport_width = aspect_ratio * viewport_height;
    // let focal_length = 1.0;

    // let origin = Vec3 {
    //     x: 0.0,
    //     y: 0.0,
    //     z: 0.0,
    // };
    // let horizontal = Vec3 {
    //     x: viewport_width,
    //     y: 0.0,
    //     z: 0.0,
    // };
    // let vertical = Vec3 {
    //     x: 0.0,
    //     y: viewport_height,
    //     z: 0.0,
    // };
    // let lower_left_corner = origin
    //     - horizontal / 2.0
    //     - vertical / 2.0
    //     - Vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: focal_length,
    //     };

    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_height) as u64)
    };

    //Render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);
            let mut pixel_color = Vec3::new();
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double_default()) / ((image_width - 1) as f64);
                let v = (j as f64 + random_double_default()) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            *pixel = image::Rgb(write_color(pixel_color, samples_per_pixel));
            //let r: f64 = (i as f64) / ((width - 1) as f64) * 255.999;
            //let g: f64 = (j as f64) / ((height - 1) as f64) * 255.999;
            //let b: f64 = 0.25 * 255.999;

            // let u = (i as f64) / ((image_width - 1) as f64);
            // let v = (j as f64) / ((image_height - 1) as f64);

            // let r = Ray {
            //     orig: origin,
            //     dir: lower_left_corner + horizontal * u + vertical * v - origin,
            // };
            // let pixel_color = ray_color(&r, &world);

            // *pixel = image::Rgb(write_color(pixel_color));
        }
        progress.inc(1);
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
