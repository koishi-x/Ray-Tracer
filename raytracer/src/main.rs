mod hittable;
mod ray;
mod sphere;
mod vec3;

use console::style;
//use image::flat::View;
use hittable::*;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use ray::*;
use sphere::*;
use std::{fs::File, process::exit};
use vec3::*;

const ZERO_VEC3: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

fn ray_color(r: Ray) -> Vec3 {
    let center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let tmp = Sphere {
        center,
        radius: 0.5,
    };
    let mut hit_record: HitRecord = HitRecord {
        p: ZERO_VEC3,
        normal: ZERO_VEC3,
        t: 0.0,
        front_face: false,
    };
    if tmp.hit(r, 0.0, 114514.0, &mut hit_record) {
        return Vec3 {
            x: hit_record.normal.x + 1.0,
            y: hit_record.normal.y + 1.0,
            z: hit_record.normal.z + 1.0,
        } * 0.5;
    }
    /*let t = hit_sphere(center, 0.5, r);

    if t > 0.0 {
        let n = unit_vector(r.at(t) - center);
        return Vec3 {
            x: n.x + 1.0,
            y: n.y + 1.0,
            z: n.z + 1.0,
        }
        .multiply(0.5);
    }*/
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
fn main() {
    let path = std::path::Path::new("output/book1/image4.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_height) as u64)
    };

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);

            //let r: f64 = (i as f64) / ((width - 1) as f64) * 255.999;
            //let g: f64 = (j as f64) / ((height - 1) as f64) * 255.999;
            //let b: f64 = 0.25 * 255.999;

            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);

            let r = Ray {
                orig: origin,
                dir: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            let pixel_color = ray_color(r);

            *pixel = image::Rgb([
                (pixel_color.x * 255.999) as u8,
                (pixel_color.y * 255.999) as u8,
                (pixel_color.z * 255.999) as u8,
            ]);
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
