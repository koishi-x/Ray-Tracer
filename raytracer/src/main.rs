#![allow(unused_imports, dead_code, unused_assignments)]

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use aabb::*;
use bvh::*;
use camera::*;
use color::*;
use hittable_list::*;
use material::*;
use moving_sphere::*;
use perlin::*;
use ray::*;
use rtweekend::*;
use sphere::*;
use texture::*;
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

    // let ground_material = Rc::new(Lambertian::new(Vec3 {
    //     x: 0.5,
    //     y: 0.5,
    //     z: 0.5,
    // }));

    // world.add(Rc::new(Sphere {
    //     center: Vec3 {
    //         x: 0.0,
    //         y: -1000.0,
    //         z: 0.0,
    //     },
    //     radius: 1000.0,
    //     mat_ptr: ground_material,
    // }));
    let checker = CheckerTexture::new(
        Color {
            x: 0.2,
            y: 0.3,
            z: 0.1,
        },
        Color {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        },
    );
    world.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Rc::new(Lambertian {
            albedo: Rc::new(checker),
        }),
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
                    let center2 = center
                        + Vec3 {
                            x: 0.,
                            y: random_double(0., 0.5),
                            z: 0.,
                        };
                    world.add(Rc::new(MovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                    // world.add(Rc::new(Sphere {
                    //     center,
                    //     radius: 0.2,
                    //     mat_ptr: sphere_material,
                    // }));
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

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Rc::new(CheckerTexture::new(
        Color {
            x: 0.2,
            y: 0.3,
            z: 0.1,
        },
        Color {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        },
    ));
    objects.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Rc::new(Lambertian {
            albedo: checker.clone(),
        }),
    }));
    objects.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Rc::new(Lambertian { albedo: checker }),
    }));
    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new());

    objects.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Rc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    }));
    objects.add(Rc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Rc::new(Lambertian { albedo: pertext }),
    }));
    objects
}
fn main() {
    //path
    let path = std::path::Path::new("output/book2/image7.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

    //World

    let world: HittableList;
    let mut lookfrom = Point3::new();
    let mut lookat = Point3::new();
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let world_type = 0;
    match world_type {
        1 => {
            world = random_scene();
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
        _ => {
            world = two_perlin_spheres();
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
    }
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
                let r = cam.get_ray(u, v, 0.0, 1.0);
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
