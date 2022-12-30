pub mod image;
pub mod kernels;

use image::ImageChannel;

pub fn clamp_reflect(value: i64, low: i64, high: i64) -> i64 {
    if value < low {
        -value
    } else if value >= high {
        high * 2 - value - 1
    } else {
        value
    }
}

/// performs image1 - image2 operation, subtracting values in image1 by values in
/// image 2
///
/// for now, just simple subtraction clamping at 0 (saturating)
pub fn subtract_images(image1: &[u8], image2: &[u8]) -> Vec<u8> {
    image1
        .iter()
        .zip(image2.iter())
        .map(|(b1, b2)| b1.saturating_sub(*b2))
        .collect()
}

/// image1 + image2 - adding values in image1 to values in image2
///
/// for now, just simple saturating addition (clamps to 255)
#[allow(unused)]
pub fn add_images(image1: &[u8], image2: &[u8]) -> Vec<u8> {
    image1
        .iter()
        .zip(image2.iter())
        .map(|(b1, b2)| b1.saturating_add(*b2))
        .collect()
}

/// applies a given kernel to an image via convolution/cross correlation
pub fn apply_filter(image_channel: &ImageChannel, kernel: &kernels::Kernel) -> ImageChannel {
    // kernel must have odd number of rows and columns
    let kern_h = kernel.data.len() as u32 / kernel.width;
    let kern_w = kernel.width;
    assert!(kern_h % 2 == 1 && kern_w % 2 == 1);

    let kern_offx: usize = (kern_w / 2).try_into().unwrap();
    let kern_offy: usize = (kern_h / 2).try_into().unwrap();

    let mut out_bytes: Vec<u8> = Vec::new();
    for i in 0..i64::from(image_channel.height) {
        for j in 0..i64::from(image_channel.width) {
            let mut sum: f64 = 0f64;
            for u in 0..kern_w {
                for v in 0..kern_h {
                    let tlx = j - kern_offx as i64;
                    let tly = i - kern_offy as i64;
                    sum += f64::from(get_image_pixel(
                        &image_channel.bytes,
                        image_channel.width as usize,
                        tlx + u as i64,
                        tly + v as i64,
                    )) * kernel.data[(u + v * kern_w) as usize];
                }
            }
            out_bytes.push(sum as u8);
        }
    }

    ImageChannel::new(image_channel.width, image_channel.height, out_bytes)
}

/// get what should be at a pixel in an image, supporting negative indices
/// by reflecting what's already in the image
fn get_image_pixel(image: &[u8], width: usize, x: i64, y: i64) -> u8 {
    let x = clamp_reflect(x, 0, width as i64);
    let y = clamp_reflect(y, 0, image.len() as i64 / width as i64);
    let x: usize = x.try_into().unwrap();
    let y: usize = y.try_into().unwrap();

    image[y * width + x]
}
