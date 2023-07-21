use std::{cmp::Ordering, hash::BuildHasherDefault};

use image::{DynamicImage, GenericImageView, GrayImage, Luma, Pixel};

use crate::*;

pub fn get_u8(pixel: &Luma<u8>) -> u8 {
    pixel.channels()[0]
}

#[derive(Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    value: f64,
}

impl Node {
    fn new(x: usize, y: usize, value: f64) -> Node {
        Node { x, y, value }
    }
}

fn gray_compare(a: &Node, b: &Node) -> Ordering {
    if a.value < b.value {
        Ordering::Less
    } else {
        Ordering::Greater
    }
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
    let mut gradients: Vec<Vec<f64>> = Vec::new();
    let mut direction: Vec<Vec<f64>> = Vec::new();

    let mut nodes = Vec::new();
    for i in 0..width as usize {
        gradients.push(Vec::new());
        direction.push(Vec::new());
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
            gradients[i].push((gx as f64 * gx as f64 + gy as f64 * gy as f64).sqrt());
            direction[i].push(gy as f64 / gx as f64);
        }
    }

    let mut gray_value: Vec<Vec<f64>> = Vec::new();
    for i in 0..width as usize {
        gray_value.push(Vec::new());
        for j in 0..height as usize {
            let cur_value;
            if i == 0 || i as u32 == width - 1 || j == 0 || j as u32 == height - 1
            // || gradients[i][j] < gradients[i - 1][j - 1]
            // || gradients[i][j] < gradients[i - 1][j]
            // || gradients[i][j] < gradients[i - 1][j + 1]
            // || gradients[i][j] < gradients[i][j - 1]
            // || gradients[i][j] < gradients[i][j + 1]
            // || gradients[i][j] < gradients[i + 1][j - 1]
            // || gradients[i][j] < gradients[i + 1][j]
            // || gradients[i][j] < gradients[i + 1][j + 1]
            {
                cur_value = 0.0;
            } else {
                let dtmp1: f64;
                let dtmp2: f64;
                if direction[i][j] >= 1.0 {
                    dtmp1 = gradients[i - 1][j] * (1.0 - 1.0 / direction[i][j])
                        + gradients[i - 1][j + 1] * (1.0 / direction[i][j]);
                    dtmp2 = gradients[i + 1][j] * (1.0 - 1.0 / direction[i][j])
                        + gradients[i + 1][j - 1] * (1.0 / direction[i][j]);
                } else if direction[i][j] >= 0.0 {
                    dtmp1 = gradients[i - 1][j + 1] * (direction[i][j])
                        + gradients[i][j + 1] * (1.0 - direction[i][j]);
                    dtmp2 = gradients[i + 1][j - 1] * (direction[i][j])
                        + gradients[i][j - 1] * (1.0 - direction[i][j]);
                } else if direction[i][j] >= -1.0 {
                    dtmp1 = gradients[i][j + 1] * (1.0 + direction[i][j])
                        + gradients[i + 1][j + 1] * (-direction[i][j]);
                    dtmp2 = gradients[i][j - 1] * (1.0 + direction[i][j])
                        + gradients[i - 1][j - 1] * (-direction[i][j]);
                } else {
                    dtmp1 = gradients[i + 1][j + 1] * (-1.0 / direction[i][j])
                        + gradients[i + 1][j] * (1.0 + 1.0 / direction[i][j]);
                    dtmp2 = gradients[i - 1][j - 1] * (-1.0 / direction[i][j])
                        + gradients[i - 1][j] * (1.0 + 1.0 / direction[i][j]);
                }
                if gradients[i][j] >= dtmp1 && gradients[i][j] > dtmp2 {
                    cur_value = gradients[i][j];
                    nodes.push(Node::new(i, j, cur_value));
                } else {
                    cur_value = 0.0;
                }
            }

            gray_value[i].push(cur_value);
        }
    }
    nodes[..].sort_by(gray_compare);
    let threshold_high = nodes[(nodes.len() as f64 * 0.8) as usize].value;
    let threshold_low = nodes[(nodes.len() as f64 * 0.4) as usize].value;
    let mut vis: Vec<Vec<bool>> = Vec::new();
    for i in 0..width as usize {
        vis.push(Vec::new());
        for _j in 0..height {
            vis[i].push(false);
        }
    }

    for i in 0..width as usize {
        for j in 0..height as usize {
            if !vis[i][j] && gray_value[i][j] >= threshold_high {
                vis[i][j] = true;
                dfs(i, j, threshold_low, width, height, &mut vis, &gray_value);
            }
        }
    }

    for (i, j, pixel) in canny_img.enumerate_pixels_mut() {
        //*pixel = image::Luma([gradients[(i * height + j) as usize] as u8]);
        //let id = (i * height + j) as usize;
        if vis[i as usize][j as usize] {
            *pixel = image::Rgb([
                clamp(
                    img.get_pixel(i, j)[0] as f64 - gray_value[i as usize][j as usize],
                    0.0,
                    255.0,
                ) as u8,
                clamp(
                    img.get_pixel(i, j)[1] as f64 - gray_value[i as usize][j as usize],
                    0.0,
                    255.0,
                ) as u8,
                clamp(
                    img.get_pixel(i, j)[2] as f64 - gray_value[i as usize][j as usize],
                    0.0,
                    255.0,
                ) as u8,
            ])
        } else {
            *pixel = image::Rgb([
                clamp(img.get_pixel(i, j)[0] as f64, 0.0, 255.0) as u8,
                clamp(img.get_pixel(i, j)[1] as f64, 0.0, 255.0) as u8,
                clamp(img.get_pixel(i, j)[2] as f64, 0.0, 255.0) as u8,
            ])
        }
    }

    canny_img.save(outpath).unwrap();
}

const DX: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const DY: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

fn dfs(
    x: usize,
    y: usize,
    low: f64,
    width: u32,
    height: u32,
    vis: &mut Vec<Vec<bool>>,
    img: &Vec<Vec<f64>>,
) {
    for i in 0..8 {
        let ux = x as i32 + DX[i];
        let uy = y as i32 + DY[i];
        if ux < 0 || uy < 0 || ux >= width as i32 || uy >= height as i32 {
            continue;
        }
        if vis[ux as usize][uy as usize] {
            continue;
        }
        if img[ux as usize][uy as usize] >= low {
            vis[ux as usize][uy as usize] = true;
            dfs(ux as usize, uy as usize, low, width, height, vis, img);
        }
    }
}
