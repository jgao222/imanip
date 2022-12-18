use std::{env::args, process::exit, fs::File, path::Path, io::BufWriter};
use png::OutputInfo;

mod kernels;
mod image;
use image::Image;

fn main() {
  let args: Vec<String> = args().collect();

  if args.len() < 4 {
    println!("Usage: [imanip] [FLAG] [FILE_PATH] [OUTPUT_PATH]");
    exit(1);
  }

  // get image
  let decoder = png::Decoder::new(File::open(args[2].clone()).expect("second argument should be file path"));
  let mut reader = decoder.read_info().unwrap();

  let mut buf = vec![0; reader.output_buffer_size()];
  let image_info = reader.next_frame(&mut buf).unwrap();

  let bytes = &buf[..image_info.buffer_size()];

  // convert to intermediate image struct?

  // hmm have to split into separate color channels and recombine

  // get operation from flag, perform operation on image
  let output_image = match args[1].chars().collect::<Vec<char>>()[1] {
    'g' => simple_gaussian_blur(bytes, &image_info),
    'G' => complex_gaussian_blur(bytes, &image_info),
    'i' => identity_image(bytes, &image_info),
    'd' => downscale_image(bytes, &image_info),
    'u' => todo!(),
    _ => {
      println!("Unrecognized flag");
      exit(1);
    }
  };

  // encode image back to png
  // copied from png crate docs: https://docs.rs/png/0.17.7/png/
  let out_path = Path::new(&args[3]);
  let file = File::create(out_path).expect("output file should have been created");
  let ref mut w = BufWriter::new(file);

  let mut encoder = png::Encoder::new(w, output_image.width, output_image.height);
  encoder.set_color(png::ColorType::Rgba);
  encoder.set_depth(png::BitDepth::Eight);
  encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
  encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
  let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
    (0.31270, 0.32900),
    (0.64000, 0.33000),
    (0.30000, 0.60000),
    (0.15000, 0.06000)
  );
  encoder.set_source_chromaticities(source_chromaticities);
  let mut writer = encoder.write_header().unwrap();

  writer.write_image_data(&output_image.bytes).expect("should have successfully written transformed image data to file");
}

fn apply_to_channels(image: &[u8], image_info: &OutputInfo, apply_func: &dyn Fn(&[u8], &OutputInfo) -> Image) -> Image {
  let num_channels = match image_info.color_type {
    png::ColorType::Rgb => 3,
    png::ColorType::Rgba => 4,
    _ => panic!("unexpected image color type")
  };

  // apply to each channel
  let mut channels: Vec<Image> = vec![];
  for i in 0..num_channels {
    let chan: Vec<u8> = image.iter().skip(i).step_by(num_channels).map(|&x| x).collect();
    let dchan: Image = apply_func(&chan, image_info);
    channels.push(dchan);
  }

  // recombine all channels
  let channel_size = channels[0].bytes.len();
  let mut out_bytes: Vec<u8> = Vec::new();
  for i in 0..channel_size {
    for chan in &channels {
      out_bytes.push(chan.bytes[i]);
    }
  }

  return Image { width: channels[0].width, height: channels[0].height, bytes: out_bytes};
}

/// applies a given kernel to an image via convolution/cross correlation
fn apply_filter(image: &[u8], image_info: &OutputInfo, kernel: &kernels::Kernel) -> Image {
  // kernel must have odd number of rows and columns
  let kern_h = kernel.data.len() as u32 / kernel.width;
  let kern_w = kernel.width;
  assert!(kern_h % 2 == 1 && kern_w % 2 == 1);

  let kern_offx: usize = (kern_w / 2).try_into().unwrap();
  let kern_offy: usize = (kern_h / 2).try_into().unwrap();

  let mut out_bytes: Vec<u8> = Vec::new();
  for i in 0..i64::from(image_info.height) {
    for j in 0..i64::from(image_info.width) {
      let mut sum: f64 = 0f64;
      for u in 0..kern_w {
        for v in 0..kern_h {
          let tlx = j - kern_offx as i64;
          let tly = i - kern_offy as i64;
          sum += f64::from(get_image_pixel(image, image_info.width as usize, tlx + u as i64, tly + v as i64)) * kernel.data[(u + v * kern_w) as usize];
        }
      }
      out_bytes.push(sum as u8);
    }
  }

  return Image {width: image_info.width, height: image_info.height, bytes: out_bytes };
}

/// get what should be at a pixel in an image, supporting negative indices
/// by reflecting what's already in the image
fn get_image_pixel(image: &[u8], width: usize, x: i64, y: i64) -> u8 {
  let x = clamp_reflect(x, 0, width as i64);
  let y = clamp_reflect(y, 0, image.len() as i64 / width as i64);
  let x: usize = x.try_into().unwrap();
  let y: usize = y.try_into().unwrap();
  return image[y * width + x];
}

fn clamp_reflect(value: i64, low: i64, high: i64) -> i64 {
  if value < low {
    -value
  } else if value >= high {
    high * 2 - value - 1
  } else {
    value
  }
}

/// downscales an image, handling splitting and recombining color channels
/// only supports RGB and RGBA
fn downscale_image(image: &[u8], image_info: &OutputInfo) -> Image {
  apply_to_channels(image, image_info, &resample_down)
}

/// performs identity transform of image, leaving it the same
fn identity_image(image: &[u8], image_info: &OutputInfo) -> Image {
  apply_to_channels(image, image_info, &|chan: &[u8], info: &OutputInfo| apply_filter(chan, info, &kernels::IDENTITY1))
}

fn simple_gaussian_blur(image: &[u8], image_info: &OutputInfo) -> Image {
  apply_to_channels(image, image_info, &|chan, info| apply_filter(chan, info, &kernels::GAUSSIAN_SIMPLE))
}

fn complex_gaussian_blur(image: &[u8], image_info: &OutputInfo) -> Image {
  let img = apply_to_channels(image, image_info, &|chan, info| apply_filter(chan, info, &kernels::GAUSSIAN_X));
  apply_to_channels(&img.bytes, image_info, &|chan, info| apply_filter(chan, info, &kernels::GAUSSIAN_Y))
}

/// downscales image by resampling every other column and row
fn resample_down(image_bytes: &[u8], info: &OutputInfo) -> Image {
  let width = info.width as usize;
  let rsb: Vec<u8> = image_bytes
                      .chunks(width) // get rows of image channel
                      .step_by(2) // skip every other row
                      .flatten() // flatten into one sequence of bytes
                      .step_by(2) // skip every other column (every other item)
                      .map(|&x| x) // deref
                      .collect();

  return Image { width: info.width / 2, height: info.height / 2, bytes: rsb };
}