use imanip::*;
use imanip::image::*;
use imanip::kernels;

/// downscales an image, handling splitting and recombining color channels
/// only supports RGB and RGBA
pub fn downscale_image(image: Image) -> Image {
    image.apply_to_channels(&resample_down)
}

/// performs identity transform of image, leaving it the same
pub fn identity_image(image: Image) -> Image {
    image.apply_to_channels(&|chan: &ImageChannel| {
        apply_filter(chan, &kernels::IDENTITY1)
    })
}

pub fn simple_gaussian_blur(image: Image) -> Image {
    image.apply_to_channels(&|chan| {
        apply_filter(chan, &kernels::GAUSSIAN_SIMPLE)
    })
}

pub fn complex_gaussian_blur(image: Image) -> Image {
    // apply a guassian filter utilizing more than just one cell in each direction
    // take advantage of symettry? property to get same effect by applying one filter on x and one on y
    let img = image.apply_to_channels(&|chan| {
        apply_filter(chan, &kernels::GAUSSIAN_X)
    });
    img.apply_to_channels(&|chan| {
        apply_filter(chan, &kernels::GAUSSIAN_Y)
    })
}

/// sharpens an image by subtracting its laplacian from itself
pub fn sharpen_image(image: Image) -> Image {
    let img_lap: Vec<u8> = image.apply_to_channels(&|chan| {
        apply_filter(chan, &kernels::LAPLACIAN)
    }).into();
    let image_bytes: Vec<u8> = image.clone().into();
    Image::new(image.width, image.height, image.color_type, &subtract_images(&image_bytes, &img_lap))
}

/// downscales image by resampling every other column and row
pub fn resample_down(channel: &ImageChannel) -> ImageChannel {
    let width = channel.width as usize;
    let rsb: Vec<u8> = channel.bytes
        .chunks(width) // get rows of image channel
        .step_by(2) // skip every other row
        .flatten() // flatten into one sequence of bytes
        .step_by(2) // skip every other column (every other item)
        .copied() // deref
        .collect();

    ImageChannel::new(channel.width / 2, channel.height / 2, rsb)
}