//Book2-final-Image

#![allow(unused_imports, dead_code, unused_assignments)]

mod aabb;
mod aabox;
mod aarect;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use std::{
    sync::mpsc::{self, Receiver},
    thread,
};

use aabb::*;
use aabox::*;
use aarect::*;
use bvh::*;
use camera::*;
use color::*;
use constant_medium::*;
use hittable_list::*;
use material::*;
use moving_sphere::*;
use perlin::*;
use ray::*;
use rtweekend::*;
use sphere::*;
use texture::*;
use vec3::*;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

fn ray_color(r: &Ray, background: Color, world: &(dyn Hittable + Send + Sync), depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new();
    }

    match world.hit(r, 0.001, INFINITY) {
        Some(rec) => {
            let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);

            match (*rec.mat_ptr).scatter(r, &rec) {
                None => emitted,
                Some((attenuation, scattered)) => {
                    emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
                }
            }
        }
        None => {
            background
            // let unit_direction = unit_vector(r.dir);
            // let t = 0.5 * (unit_direction.y + 1.0);
            // Vec3 {
            //     x: 1.0,
            //     y: 1.0,
            //     z: 1.0,
            // } * (1.0 - t)
            //     + Vec3 {
            //         x: 0.5,
            //         y: 0.7,
            //         z: 1.0,
            //     } * t
        }
    }
}

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();

    // let ground_material = Arc::new(Lambertian::new(Vec3 {
    //     x: 0.5,
    //     y: 0.5,
    //     z: 0.5,
    // }));

    // world.add(Arc::new(Sphere {
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
    world.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: Arc::new(checker),
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
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center
                        + Vec3 {
                            x: 0.,
                            y: random_double(0., 0.5),
                            z: 0.,
                        };
                    world.add(Arc::new(MovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                    // world.add(Arc::new(Sphere {
                    //     center,
                    //     radius: 0.2,
                    //     mat_ptr: sphere_material,
                    // }));
                } else if choose_mat < 0.95 {
                    //metal
                    let aldebo = random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(aldebo, fuzz));
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else {
                    //glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material1,
    }));

    let material2 = Arc::new(Lambertian::new(Vec3 {
        x: 0.4,
        y: 0.2,
        z: 0.1,
    }));
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material2,
    }));

    let material3 = Arc::new(Metal::new(
        Vec3 {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        0.0,
    ));
    world.add(Arc::new(Sphere {
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
    let checker = Arc::new(CheckerTexture::new(
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
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: checker.clone(),
        }),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian { albedo: checker }),
    }));
    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian { albedo: pertext }),
    }));
    objects
}

fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new("input/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_texture(earth_texture));

    let mut objects = HittableList::new();
    objects.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: earth_surface,
    }));
    objects
}

fn simple_light() -> HittableList {
    let mut objects: HittableList = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian::new_texture(pertext.clone())),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian::new_texture(pertext)),
    }));

    let difflight = Arc::new(DiffuseLight::new(Color {
        x: 4.0,
        y: 4.0,
        z: 4.0,
    }));
    objects.add(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    )));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 7.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: difflight,
    }));
    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::new(Vec3 {
        x: 0.65,
        y: 0.05,
        z: 0.05,
    }));
    let white = Arc::new(Lambertian::new(Vec3 {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    }));
    let green = Arc::new(Lambertian::new(Vec3 {
        x: 0.12,
        y: 0.45,
        z: 0.15,
    }));
    let light = Arc::new(DiffuseLight::new(Vec3 {
        x: 15.0,
        y: 15.0,
        z: 15.0,
    }));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable + Send + Sync> = Arc::new(AABox::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 165.0,
            y: 330.0,
            z: 165.0,
        },
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(
        box1,
        Vec3 {
            x: 265.0,
            y: 0.0,
            z: 295.0,
        },
    ));
    objects.add(box1);

    let mut box2: Arc<dyn Hittable + Send + Sync> = Arc::new(AABox::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 165.0,
            y: 165.0,
            z: 165.0,
        },
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(
        box2,
        Vec3 {
            x: 130.0,
            y: 0.0,
            z: 65.0,
        },
    ));
    objects.add(box2);

    objects
    // let mut bvh = HittableList::new();
    // bvh.add(Arc::new(BvhNode::new_hittablelist(objects, 0.0, 1.0)));
    // bvh
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::new(Vec3 {
        x: 0.65,
        y: 0.05,
        z: 0.05,
    }));
    let white = Arc::new(Lambertian::new(Vec3 {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    }));
    let green = Arc::new(Lambertian::new(Vec3 {
        x: 0.12,
        y: 0.45,
        z: 0.15,
    }));
    let light = Arc::new(DiffuseLight::new(Vec3 {
        x: 7.0,
        y: 7.0,
        z: 7.0,
    }));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable + Send + Sync> = Arc::new(AABox::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 165.0,
            y: 330.0,
            z: 165.0,
        },
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(
        box1,
        Vec3 {
            x: 265.0,
            y: 0.0,
            z: 295.0,
        },
    ));

    let mut box2: Arc<dyn Hittable + Send + Sync> = Arc::new(AABox::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 165.0,
            y: 165.0,
            z: 165.0,
        },
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(
        box2,
        Vec3 {
            x: 130.0,
            y: 0.0,
            z: 65.0,
        },
    ));

    objects.add(Arc::new(ConstantMedium::new_color(
        box1,
        0.01,
        Color {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    )));
    objects.add(Arc::new(ConstantMedium::new_color(
        box2,
        0.01,
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    )));
    objects
}

fn final_scene() -> HittableList {
    let mut objects = HittableList::new();
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color {
        x: 0.48,
        y: 0.83,
        z: 0.53,
    }));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(AABox::new(
                Point3 {
                    x: x0,
                    y: y0,
                    z: z0,
                },
                Point3 {
                    x: x1,
                    y: y1,
                    z: z1,
                },
                ground.clone(),
            )));
        }
    }
    objects.add(Arc::new(BvhNode::new_hittablelist(boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new(Color {
        x: 7.0,
        y: 7.0,
        z: 7.0,
    }));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center0 = Point3 {
        x: 400.0,
        y: 400.0,
        z: 200.0,
    };
    let center1 = center0
        + Vec3 {
            x: 30.0,
            y: 0.0,
            z: 0.0,
        };
    let moving_sphere_material = Arc::new(Lambertian::new(Color {
        x: 0.7,
        y: 0.3,
        z: 0.1,
    }));
    objects.add(Arc::new(MovingSphere {
        center0,
        center1,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: moving_sphere_material,
    }));

    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 260.0,
            y: 150.0,
            z: 45.0,
        },
        radius: 50.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 150.0,
            z: 145.0,
        },
        radius: 50.0,
        mat_ptr: Arc::new(Metal::new(
            Color {
                x: 0.8,
                y: 0.8,
                z: 0.9,
            },
            1.0,
        )),
    }));

    let boundary = Arc::new(Sphere {
        center: Point3 {
            x: 360.0,
            y: 150.0,
            z: 145.0,
        },
        radius: 70.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    });
    objects.add(boundary.clone());

    objects.add(Arc::new(ConstantMedium::new_color(
        boundary,
        0.2,
        Color {
            x: 0.2,
            y: 0.4,
            z: 0.9,
        },
    )));
    objects.add(Arc::new(ConstantMedium::new_color(
        Arc::new(Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 5000.0,
            mat_ptr: Arc::new(Dielectric::new(1.5)),
        }),
        0.0001,
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    )));

    let emat = Arc::new(Lambertian::new_texture(Arc::new(ImageTexture::new(
        "input/earthmap.jpg",
    ))));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 400.0,
            y: 200.0,
            z: 400.0,
        },
        radius: 100.0,
        mat_ptr: emat,
    }));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 220.0,
            y: 280.0,
            z: 300.0,
        },
        radius: 80.0,
        mat_ptr: Arc::new(Lambertian::new_texture(pertext)),
    }));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    }));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere {
            center: random(0.0, 165.0),
            radius: 10.0,
            mat_ptr: white.clone(),
        }));
    }
    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new_hittablelist(boxes2, 0.0, 1.0)),
            15.0,
        )),
        Point3 {
            x: -100.0,
            y: 270.0,
            z: 395.0,
        },
    )));

    objects
}

fn main() {
    //path
    let path = std::path::Path::new("output/book2/image22.jpg");
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

    let world_type = 0;
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
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            lookfrom = Point3 {
                x: 278.0,
                y: 278.0,
                z: -800.0,
            };
            lookat = Point3 {
                x: 278.0,
                y: 278.0,
                z: 0.0,
            };
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            lookfrom = Point3 {
                x: 278.0,
                y: 278.0,
                z: -800.0,
            };
            lookat = Point3 {
                x: 278.0,
                y: 278.0,
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
        let pb = multi_progress.add(ProgressBar::new(task.len() as u64));
        let cur_thread: thread::JoinHandle<()> = thread::spawn(move || {
            for (i, j) in task {
                let mut pixel_color = Vec3::new();

                for _s in 0..samples_per_pixel {
                    let u = (i as f64 + random_double_default()) / ((image_width - 1) as f64);
                    let v = (j as f64 + random_double_default()) / ((image_height - 1) as f64);
                    let r = cam.get_ray(u, v, 0.0, 1.0);
                    pixel_color += ray_color(&r, background, &world_tmp, max_depth);
                }
                tx.send(ColorInformation::new(i, image_height - j - 1, pixel_color))
                    .unwrap();
                pb.inc(1);
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

    exit(0);
}
