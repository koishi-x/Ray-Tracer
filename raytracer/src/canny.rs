use std::hash::BuildHasherDefault;

use image::{DynamicImage, GenericImageView, GrayImage, Luma, Pixel};

use crate::*;

pub fn get_u8(pixel: &Luma<u8>) -> u8 {
    pixel.channels()[0]
}

pub fn canny_check(path: &str, outpath: &str) {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();

    let mut gray_img: Vec<Vec<f64>> = Vec::new();
    for i in 0..width {
        gray_img.push(Vec::new());
        for j in 0..height {
            gray_img[i as usize].push(
                img.get_pixel(i, j)[0] as f64 * 0.299
                    + img.get_pixel(i, j)[1] as f64 * 0.587
                    + img.get_pixel(i, j)[2] as f64 * 0.114,
            );
        }
    }

    let mut canny_img: RgbImage = ImageBuffer::new(width, height);

    let mut x_gradients = Vec::new();
    let mut y_gradients = Vec::new();
    let mut gradients = Vec::new();
    let mut direction = Vec::new();
    for i in 0..width as usize {
        for j in 0..height as usize {
            let gx: f64;
            let gy: f64;
            if i == 0 || i as u32 == width - 1 || j == 0 || j as u32 == height - 1 {
                gx = 0.;
                gy = 0.;
            } else {
                gx = 2.0 * (gray_img[i][j + 1] - gray_img[i][j - 1]) + gray_img[i - 1][j + 1]
                    - gray_img[i - 1][j - 1]
                    + gray_img[i + 1][j + 1]
                    - gray_img[i + 1][j - 1];

                gy = 2.0 * (gray_img[i - 1][j] - gray_img[i + 1][j]) + gray_img[i - 1][j - 1]
                    - gray_img[i + 1][j - 1]
                    + gray_img[i - 1][j + 1]
                    - gray_img[i + 1][j + 1];
            }
            x_gradients.push(gx);
            y_gradients.push(gy);
            gradients.push((gx as f64 * gx as f64 + gy as f64 * gy as f64).sqrt());
            direction.push(gy as f64 / gx as f64);
        }
    }

    for (i, j, pixel) in canny_img.enumerate_pixels_mut() {
        //*pixel = image::Luma([gradients[(i * height + j) as usize] as u8]);
        let id = (i * height + j) as usize;
        *pixel = image::Rgb([
            clamp(img.get_pixel(i, j)[0] as f64 - gradients[id], 0.0, 255.0) as u8,
            clamp(img.get_pixel(i, j)[1] as f64 - gradients[id], 0.0, 255.0) as u8,
            clamp(img.get_pixel(i, j)[2] as f64 - gradients[id], 0.0, 255.0) as u8,
        ])
    }

    canny_img.save(outpath).unwrap();
}
