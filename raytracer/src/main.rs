#![allow(unused_imports, dead_code, unused_assignments)]

mod aabb;
mod aabox;
mod aarect;
mod bvh;
mod camera;
mod canny;
mod color;
mod constant_medium;
mod hittable_list;
mod image_box;
mod material;
mod moving_sphere;
mod obj_loader;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rtweekend;
mod scene;
mod sphere;
mod texture;
mod triangle;
mod vec3;

use std::{
    ops::Deref,
    sync::mpsc::{self, Receiver},
    thread,
};

use aabb::*;
use aabox::*;
use aarect::*;
use bvh::*;
use camera::*;
use canny::*;
use color::*;
use constant_medium::*;
use hittable_list::*;
use image_box::*;
use material::*;
use moving_sphere::*;
use obj_loader::*;
use onb::*;
use pdf::*;
use perlin::*;
use ray::*;
use rtweekend::*;
use scene::*;
use sphere::*;
use texture::*;
use triangle::*;
use vec3::*;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

fn ray_color<'a, H: Hittable>(
    r: &Ray,
    background: Color,
    world: &'a HittableList,
    lights: &'a H,
    depth: i32,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::new();
    }

    match world.hit(r, 0.001, INFINITY) {
        Some(rec) => {
            let emitted = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, rec.p);
            //let mut pdf: f64 = 0.0;

            match (*rec.mat_ptr).scatter(r, &rec) {
                None => emitted,
                Some(srec) => {
                    if srec.is_specular {
                        return srec.attenuation
                            * ray_color(&srec.specular_ray, background, world, lights, depth - 1);
                    }

                    let light_ptr = HittablePdf::new(lights, rec.p);

                    let p = MixturePdf::new(srec.pdf_ptr.as_ref(), &light_ptr);
                    //let p = srec.pdf_ptr.as_ref();
                    let scattered = Ray::new_tm(rec.p, p.generate(), r.tm);
                    let pdf_val = p.value(scattered.dir);

                    emitted
                        + srec.attenuation
                            * ray_color(&scattered, background, world, lights, depth - 1)
                            * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                            / pdf_val
                }
            }
        }
        None => background,
    }
}

fn main() {
    // loop_work();
    // exit(1);

    let path_str = "output/final_scene.jpg";

    // let edge_check_str = "output/book0/image0_2_edge.jpg";
    // canny_check(path_str, edge_check_str);
    // exit(1);

    //path
    let path = std::path::Path::new(path_str);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let mut aspect_ratio: f64 = 16.0 / 9.0;
    let mut image_width: u32 = 400;
    let mut samples_per_pixel: u32 = 100;
    let max_depth = 50;

    //World

    let mut world = HittableList::new();
    let mut lookfrom = Point3::new();
    let mut lookat = Point3::new();
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Color::new();

    let mut lights_list = HittableList::new();
    lights_list.add(Arc::new(XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Lambertian::new(Vec3::new()),
    )));
    // lights_list.add(Arc::new(Sphere {
    //     center: Vec3 {
    //         x: 278.0,
    //         y: 25.0,
    //         z: 100.0,
    //     },
    //     radius: 25.0,
    //     mat_ptr: Lambertian::new(Vec3::new()),
    // }));
    let lights = lights_list;

    let world_type = 6;
    match world_type {
        1 => {
            world = random_scene();
            background = Color {
                x: 0.7,
                y: 0.8,
                z: 1.0,
            };
            lookfrom = Point3 {
                x: 13.0,
                y: 2.0,
                z: 3.0,
            };
            lookat = Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            background = Color {
                x: 0.7,
                y: 0.8,
                z: 1.0,
            };
            lookfrom = Point3 {
                x: 13.0,
                y: 2.0,
                z: 3.0,
            };
            lookat = Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Color {
                x: 0.7,
                y: 0.8,
                z: 1.0,
            };
            lookfrom = Point3 {
                x: 13.0,
                y: 2.0,
                z: 3.0,
            };
            lookat = Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = Color {
                x: 0.7,
                y: 0.8,
                z: 1.0,
            };
            lookfrom = Point3 {
                x: 13.0,
                y: 2.0,
                z: 3.0,
            };
            lookat = Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            background = Color::new();
            samples_per_pixel = 400;
            background = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            lookfrom = Point3 {
                x: 26.0,
                y: 3.0,
                z: 6.0,
            };
            lookat = Point3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            };
            vfov = 20.0
        }
        6 => {
            world = arknights();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            // background = Color {
            //     x: 0.7,
            //     y: 0.8,
            //     z: 0.9,
            // };

            lookfrom = Point3 {
                x: 478.0,
                y: 278.0,
                z: -600.0,
            };
            lookat = Point3 {
                x: 278.0,
                y: 278.0,
                z: 0.0,
            };
            vfov = 40.0;
        }
        7 => {
            world = test_ground();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 100;
            background = Color {
                x: 0.7,
                y: 0.8,
                z: 0.9,
            };
            lookfrom = Point3 {
                x: -100.0,
                y: 65.0,
                z: 0.,
            };
            lookat = Point3 {
                x: 0.0,
                y: 60.0,
                z: 0.0,
            };
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            background = Color::new();
            lookfrom = Point3 {
                x: 478.0,
                y: 278.0,
                z: -600.0,
            };
            lookat = Point3 {
                x: 278.0,
                y: 278.0,
                z: 0.0,
            };
            vfov = 40.0;
        }
    }
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    //Camera

    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    const THREAD_NUM: u32 = 16;
    const PROGRESS_INC_NUM: u64 = 100;

    let multi_progress = MultiProgress::new();

    let mut receivers = Vec::new();

    let mut task_list: Vec<Vec<(u32, u32)>> = vec![Vec::new(); THREAD_NUM as usize];
    for i in 0..image_width {
        for j in 0..image_height {
            task_list[(i % THREAD_NUM) as usize].push((i, j));
        }
    }

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for task in task_list {
        let (tx, rx) = mpsc::channel();
        receivers.push(rx);
        let world_tmp = world.clone();
        let lights_tmp = lights.clone();
        let pb = multi_progress.add(ProgressBar::new(task.len() as u64 / PROGRESS_INC_NUM));
        let mut count = 0;
        let cur_thread: thread::JoinHandle<()> = thread::spawn(move || {
            for (i, j) in task {
                let mut pixel_color = Vec3::new();

                for _s in 0..samples_per_pixel {
                    let u = (i as f64 + random_double_default()) / ((image_width - 1) as f64);
                    let v = (j as f64 + random_double_default()) / ((image_height - 1) as f64);
                    let r = cam.get_ray(u, v, 0.0, 1.0);
                    pixel_color += ray_color(&r, background, &world_tmp, &lights_tmp, max_depth);
                }
                tx.send(ColorInformation::new(i, image_height - j - 1, pixel_color))
                    .unwrap();
                count += 1;
                if count == PROGRESS_INC_NUM {
                    pb.inc(1);
                    count = 0;
                }
            }
            pb.finish();
        });
        threads.push(cur_thread);
    }
    multi_progress.join_and_clear().unwrap();

    for rx in receivers {
        for received in rx {
            let pixel = img.get_pixel_mut(received.i, received.j);
            *pixel = image::Rgb(write_color(received.color, samples_per_pixel));
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }
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

    //canny_check(path_str, edge_check_str);
    exit(0);
}
