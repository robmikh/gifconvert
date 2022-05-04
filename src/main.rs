mod args;
mod file;

use std::{path::PathBuf, str::FromStr};

use args::Args;
use clap::StructOpt;
use file::{create_storage_file_from_path, get_full_path_name};
use windows::{
    core::Result,
    Graphics::Imaging::{BitmapDecoder, BitmapEncoder},
    Storage::{CreationCollisionOption, FileAccessMode, StorageFile},
    Win32::System::WinRT::{RoInitialize, RO_INIT_MULTITHREADED},
};

fn main() -> Result<()> {
    unsafe {
        RoInitialize(RO_INIT_MULTITHREADED)?;
    }

    let args = Args::parse();

    let input_full_path = get_full_path_name(&args.input)?;
    let input_file = StorageFile::GetFileFromPathAsync(input_full_path)?.get()?;

    let output_path = if let Some(output) = args.output {
        output
    } else {
        let mut temp_path = PathBuf::from_str(&args.input).unwrap();
        temp_path.set_extension("gif");
        temp_path.to_str().unwrap().to_owned()
    };
    let output_file =
        create_storage_file_from_path(&output_path, CreationCollisionOption::ReplaceExisting)?;

    let (bytes, width, height, format, alpha_mode) = {
        let stream = input_file.OpenReadAsync()?.get()?;
        let decoder = BitmapDecoder::CreateAsync(stream)?.get()?;
        let frame = decoder.GetFrameAsync(0)?.get()?;
        let width = frame.PixelWidth()?;
        let height = frame.PixelHeight()?;
        let format = frame.BitmapPixelFormat()?;
        let alpha_mode = frame.BitmapAlphaMode()?;

        let pixel_data = frame.GetPixelDataAsync()?.get()?;
        let bytes = pixel_data.DetachPixelData()?;

        (bytes, width, height, format, alpha_mode)
    };

    {
        let stream = output_file.OpenAsync(FileAccessMode::ReadWrite)?.get()?;
        let encoder = BitmapEncoder::CreateAsync(BitmapEncoder::GifEncoderId()?, stream)?.get()?;
        encoder.SetPixelData(format, alpha_mode, width, height, 1.0, 1.0, &bytes)?;
        encoder.FlushAsync()?.get()?;
    }

    Ok(())
}
