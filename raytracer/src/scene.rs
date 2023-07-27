use crate::*;

pub fn random_scene() -> HittableList {
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
        mat_ptr: Lambertian { albedo: checker },
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
                    let sphere_material = Lambertian::new(albedo);
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
                    let sphere_material = Metal::new(aldebo, fuzz);
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else {
                    //glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material1,
    }));

    let material2 = Lambertian::new(Vec3 {
        x: 0.4,
        y: 0.2,
        z: 0.1,
    });
    world.add(Arc::new(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: material2,
    }));

    let material3 = Metal::new(
        Vec3 {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        0.0,
    );
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

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
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
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Lambertian {
            albedo: checker.clone(),
        },
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Lambertian { albedo: checker },
    }));
    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = NoiseTexture::new(4.0);

    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Lambertian {
            albedo: pertext.clone(),
        },
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Lambertian { albedo: pertext },
    }));
    objects
}

pub fn earth() -> HittableList {
    //let earth_texture = ImageTexture::new("input/earthmap.jpg");
    //let earth_surface = Lambertian::new_texture(earth_texture);

    let mut objects = HittableList::new();
    objects.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Lambertian::new_texture(ImageTexture::new("input/earthmap.jpg")),
    }));
    objects
}

pub fn simple_light() -> HittableList {
    let mut objects: HittableList = HittableList::new();

    let pertext = NoiseTexture::new(4.0);
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Lambertian::new_texture(pertext.clone()),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Lambertian::new_texture(pertext),
    }));

    let difflight = DiffuseLight::new(Color {
        x: 4.0,
        y: 4.0,
        z: 4.0,
    });
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

pub fn arknights() -> HittableList {
    let mut objects = HittableList::new();
    let aluminum = Metal::new(
        Color {
            x: 0.8,
            y: 0.85,
            z: 0.88,
        },
        0.0,
    );

    let box_min = Point3 {
        x: -540.0,
        y: 0.0,
        z: 0.0,
    };
    let box_max = Point3 {
        x: -490.0,
        y: 150.0,
        z: 450.0,
    };

    objects.add(Arc::new(RotateY::new(
        YZImageBox::new(box_min, box_max, "input/1.png", 0.8),
        50.0,
    )));

    objects.add(Arc::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -10000.0,
            z: 0.0,
        },
        radius: 10000.0,
        mat_ptr: aluminum,
    }));

    objects.add(Arc::new(ConstantMedium::new_color(
        Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 5000.0,
            mat_ptr: Dielectric::new(1.5),
        },
        0.0001,
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    )));

    let light = DiffuseLight::new(Color {
        x: 7.0,
        y: 7.0,
        z: 7.0,
    });
    objects.add(Arc::new(FlipFace::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    ))));

    let suzuran = load_obj("obj/Suzuran/Suzuran.obj", 15.0);

    objects.add(Arc::new(Translate::new(
        RotateY::new(suzuran, 30.0),
        Vec3 {
            x: 420.0,
            y: 0.0,
            z: 1000.0,
        },
    )));

    let shamare = load_obj("obj/shamare/Shamare.obj", 15.0);

    objects.add(Arc::new(Translate::new(
        RotateY::new(shamare, -80.0),
        Vec3 {
            x: 70.0,
            y: 0.0,
            z: 1150.0,
        },
    )));

    objects
}
pub fn final_scene() -> HittableList {
    let mut objects = HittableList::new();
    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new(Color {
        x: 0.48,
        y: 0.83,
        z: 0.53,
    });

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

    let light = DiffuseLight::new(Color {
        x: 7.0,
        y: 7.0,
        z: 7.0,
    });
    objects.add(Arc::new(FlipFace::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    ))));

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
    let moving_sphere_material = Lambertian::new(Color {
        x: 0.7,
        y: 0.3,
        z: 0.1,
    });
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
        mat_ptr: Dielectric::new(1.5),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 150.0,
            z: 145.0,
        },
        radius: 50.0,
        mat_ptr: Metal::new(
            Color {
                x: 0.8,
                y: 0.8,
                z: 0.9,
            },
            1.0,
        ),
    }));

    let boundary = Sphere {
        center: Point3 {
            x: 360.0,
            y: 150.0,
            z: 145.0,
        },
        radius: 70.0,
        mat_ptr: Dielectric::new(1.5),
    };
    objects.add(Arc::new(boundary.clone()));

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
        Sphere {
            center: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 5000.0,
            mat_ptr: Dielectric::new(1.5),
        },
        0.0001,
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    )));

    let emat = Lambertian::new_texture(ImageTexture::new("input/earthmap.jpg"));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 400.0,
            y: 200.0,
            z: 400.0,
        },
        radius: 100.0,
        mat_ptr: emat,
    }));
    let pertext = NoiseTexture::new(0.1);
    objects.add(Arc::new(Sphere {
        center: Point3 {
            x: 220.0,
            y: 280.0,
            z: 300.0,
        },
        radius: 80.0,
        mat_ptr: Lambertian::new_texture(pertext),
    }));

    let mut boxes2 = HittableList::new();
    let white = Lambertian::new(Color {
        x: 0.73,
        y: 0.73,
        z: 0.73,
    });
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere {
            center: random(0.0, 165.0),
            radius: 10.0,
            mat_ptr: white.clone(),
        }));
    }
    objects.add(Arc::new(Translate::new(
        RotateY::new(BvhNode::new_hittablelist(boxes2, 0.0, 1.0), 15.0),
        Point3 {
            x: -100.0,
            y: 270.0,
            z: 395.0,
        },
    )));

    objects
}

pub fn test_ground() -> HittableList {
    let mut objects = HittableList::new();
    let test = load_obj("obj/gensokyo_test/1.obj", 1.0);

    // objects.add(Arc::new(Translate::new(
    //     RotateY::new(test, 0.0),
    //     Vec3 {
    //         x: 420.0,
    //         y: -400.0,
    //         z: 400.0,
    //     },
    // )));
    objects.add(Arc::new(test));
    objects
}
