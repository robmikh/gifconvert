mod args;

use std::{fs::File, path::PathBuf, str::FromStr};

use args::Args;
use clap::StructOpt;
use image::EncodableLayout;

fn main() {
    let args = Args::parse();

    let input_path = PathBuf::from_str(&args.input).expect("Could not parse input path!");
    let output_path = if let Some(output) = args.output {
        PathBuf::from_str(&output).expect("Could not parse output path!")
    } else {
        let mut path = input_path.clone();
        path.set_extension("gif");
        path
    };

    println!("Opening input file...");
    let input_image = image::open(input_path).expect("Could not open input file!");
    let width = input_image.width();
    let height = input_image.height();
    let input_rgba = input_image
        .as_rgba8()
        .expect("Could not interpret input file as an RGBA8 image!");
    let mut input_bytes: Vec<u8> = input_rgba.as_bytes().iter().map(|x| *x).collect();
    println!("Input file read, found {} x {} image.", width, height);

    println!("Creating output file...");
    let mut output_file = File::create(&output_path).expect("Could not create output file!");
    println!("Creating gif data...");
    let mut encoder = gif::Encoder::new(&mut output_file, width as u16, height as u16, &[])
        .expect("Could not intialize gif encoder!");
    let frame = gif::Frame::from_rgba(width as u16, height as u16, &mut input_bytes);
    encoder
        .write_frame(&frame)
        .expect("Could not write gif frame!");
    println!("Done!");

    let canonicalized_output_path = output_path.canonicalize().unwrap();
    println!(
        "Output file written to \"{}\"",
        canonicalized_output_path.display()
    );
}
