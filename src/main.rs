use std::{env::args, fs::File, io::BufWriter, path::Path, process::exit};

mod image_ops;
use image_ops::*;
use imanip::image::*;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 4 {
        println!("Usage: [imanip] [FLAG] [FILE_PATH] [OUTPUT_PATH]");
        exit(1);
    }

    // get image
    let decoder = png::Decoder::new(
        File::open(args[2].clone()).expect("second argument should be file path"),
    );
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size()];
    let image_info = reader.next_frame(&mut buf).unwrap();

    let bytes = &buf[..image_info.buffer_size()];

    // convert to intermediate image struct?
    let image = Image::new(image_info.width, image_info.height, image_info.color_type, bytes);

    // hmm have to split into separate color channels and recombine

    // get operation from flag, perform operation on image
    let output_image = match args[1].chars().collect::<Vec<char>>()[1] {
        'g' => simple_gaussian_blur(image),
        'G' => complex_gaussian_blur(image),
        'i' => identity_image(image),
        'd' => downscale_image(image),
        's' => sharpen_image(image),
        'u' => todo!(),
        _ => {
            println!("Unrecognized flag");
            exit(1);
        }
    };
    let output_bytes: Vec<u8> = output_image.clone().into();

    // encode image back to png
    // copied from png crate docs: https://docs.rs/png/0.17.7/png/
    let out_path = Path::new(&args[3]);
    let file = File::create(out_path).expect("output file should have been created");
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, output_image.width, output_image.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer
        .write_image_data(&output_bytes)
        .expect("should have successfully written transformed image data to file");
}
