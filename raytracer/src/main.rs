mod camera;
mod color;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::*;
use color::*;
use hittable_list::*;
use material::*;
use ray::*;
use rtweekend::*;
use sphere::*;
use vec3::*;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new();
    }

    match world.hit(r, 0.001, INFINITY) {
        Some(rec) => match (*rec.mat_ptr).scatter(r, &rec) {
            None => Vec3::new(),
            Some((attenuation, scattered)) => attenuation * ray_color(&scattered, world, depth - 1),
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

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Vec3 {
        x: 0.5,
        y: 0.5,
        z: 0.5,
    }));

    world.add(Rc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double_default();
            let center = Vec3 {
                x: a as f64 + 0.9 * random_double_default(),
                y: 0.2,
                z: b as f64 + 0.9 * random_double_default(),
            };

            if (center
                - Vec3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .length()
                > 0.9
            {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = random(0.0, 1.0) * random(0.0, 1.0);
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    //metal
                    let aldebo = random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(aldebo, fuzz));
                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else {
                    //glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material1,
    }));

    let material2 = Rc::new(Lambertian::new(Vec3 {
        x: 0.4,
        y: 0.2,
        z: 0.1,
    }));
    world.add(Rc::new(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material2,
    }));

    let material3 = Rc::new(Metal::new(
        Vec3 {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        0.0,
    ));
    world.add(Rc::new(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material3,
    }));

    world
}

fn main() {
    //path
    let path = std::path::Path::new("output/book1/image21.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 500;
    let max_depth = 50;

    //World

    let world = random_scene();

    //Camera

    let lookfrom = Vec3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let lookat = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
                pixel_color += ray_color(&r, &world, max_depth);
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
