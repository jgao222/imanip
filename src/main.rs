use std::{env::args, process::exit, fs::File, path::Path, io::BufWriter};

use png::OutputInfo;

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
  let output_bytes = match args[1].chars().collect::<Vec<char>>()[1] {
    'f' => todo!(),
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

  let mut encoder = png::Encoder::new(w, image_info.width / 2, image_info.height / 2);
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

  writer.write_image_data(&output_bytes).expect("should have successfully written transformed image data to file");
}

/// downscales an image, handling splitting and recombining color channels
/// only supports RGB and RGBA
fn downscale_image(image: &[u8], image_info: &OutputInfo) -> Vec<u8> {
  let num_channels = match image_info.color_type {
    png::ColorType::Rgb => 3,
    png::ColorType::Rgba => 4,
    _ => panic!("unexpected image color type")
  };

  // resample each channel down
  let mut channels: Vec<Vec<u8>> = vec![];
  for i in 0..num_channels {
    let chan: Vec<u8> = image.iter().skip(i).step_by(num_channels).map(|&x| x).collect();
    let dchan: Vec<u8> = resample_down(&chan, image_info.width.try_into().unwrap());
    channels.push(dchan);
  }

  // recombine all channels
  let channel_size = channels[0].len();
  let mut out: Vec<u8> = Vec::new();
  for i in 0..channel_size {
    for chan in &channels {
      out.push(chan[i]);
    }
  }

  return out;
}

/// downscales image by resampling every other column and row
fn resample_down(image_bytes: &[u8], width: usize) -> Vec<u8> {
  let rsb: Vec<u8> = image_bytes
                      .chunks(width) // get rows of image channel
                      .step_by(2) // skip every other row
                      .flatten() // flatten into one sequence of bytes
                      .step_by(2) // skip every other column (every other item)
                      .map(|&x| x) // deref
                      .collect();

  return rsb;
}