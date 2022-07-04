mod args;

use std::{fs::File, path::PathBuf, str::FromStr};

use args::Args;
use clap::StructOpt;
use image::EncodableLayout;

fn main() {
    let args = Args::parse();

    let input_path = &args.input;
    let output_path = if let Some(output) = args.output {
        output
    } else {
        let mut temp_path = PathBuf::from_str(&args.input).unwrap();
        temp_path.set_extension("gif");
        temp_path.to_str().unwrap().to_owned()
    };

    let input_image = image::open(input_path).unwrap();
    let width = input_image.width();
    let height = input_image.height();
    let input_rgba = input_image.as_rgba8().unwrap();
    let mut input_bytes: Vec<u8> = input_rgba.as_bytes().iter().map(|x| *x).collect();

    let mut output_file = File::create(output_path).unwrap();
    let mut encoder =
        gif::Encoder::new(&mut output_file, width as u16, height as u16, &[]).unwrap();
    let frame = gif::Frame::from_rgba(width as u16, height as u16, &mut input_bytes);
    encoder.write_frame(&frame).unwrap();
}
