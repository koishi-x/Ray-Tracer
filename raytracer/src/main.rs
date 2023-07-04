use console::style;
//use image::flat::View;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::ops::{Add, BitXor, Mul, Sub};
use std::{fs::File, process::exit};

#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl BitXor for Vec3 {
    //cross product
    type Output = Vec3;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Mul for Vec3 {
    //dot product
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y + rhs.y + self.z * rhs.z
    }
}

impl Vec3 {
    /*
    fn minus(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
    fn plus_self(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    fn subtract_self(&mut self, other: &Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
    fn mutiply_self(&mut self, num: f64) {
        self.x *= num;
        self.y *= num;
        self.z *= num;
    }
    fn divide_self(&mut self, num: f64) {
        self.x /= num;
        self.y /= num;
        self.z /= num;
    }
    fn plus(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    fn subtract(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }*/
    fn multiply(&self, num: f64) -> Vec3 {
        Vec3 {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
        }
    }
    fn divide(&self, num: f64) -> Vec3 {
        Vec3 {
            x: self.x / num,
            y: self.y / num,
            z: self.z / num,
        }
    }
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

fn unit_vector(v: &Vec3) -> Vec3 {
    v.divide(v.length())
}
struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    /*
    fn at(&self, t: f64) -> Vec3 {
        //self.orig.plus(&self.dir.multiply(t))
        self.orig + self.dir.multiply(t)
    }
    */
}

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = unit_vector(&r.dir) + r.orig.multiply(0.0);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }
    .multiply(1.0 - t)
        + Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
        .multiply(t)
}
fn main() {
    let path = std::path::Path::new("output/book1/image2.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = image_width / (aspect_ratio as u32);

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
        - horizontal.divide(2.0)
        - vertical.divide(2.0)
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
                dir: lower_left_corner + horizontal.multiply(u) + vertical.multiply(v) - origin,
            };
            let pixel_color = ray_color(&r);

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
