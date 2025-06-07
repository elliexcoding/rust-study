use libheif_rs::{Channel, ColorSpace, HeifContext, ItemId, LibHeif, Result, RgbChroma};
use std::ffi;
use std::ptr;
use image::{DynamicImage, ImageReader, Rgb, ExtendedColorType, ImageEncoder};
use image::codecs::{avif::AvifEncoder};
use std::io::Cursor;


pub fn encode_webp(image: &DynamicImage) -> Result<Vec<u8>> {
    // Convert the image to RGBA format
    let rgba_image = image.to_rgba8();
    let mut webp_data = Vec::new();
    // Create a WebP encoder (support only lossless feature for now)
    let encoder = AvifEncoder::new(&mut webp_data);
    encoder.write_image(
        &rgba_image,
        rgba_image.width(),
        rgba_image.height(),
        ExtendedColorType::Rgba8,
    ).expect("TODO: panic message");

    Ok(webp_data)
}

fn main() -> Result<()>{
    println!("Hello, world!");
    let lib = LibHeif::new();
    let ctx = HeifContext::read_from_file(
        "./data/IMG_1415.HEIC"
    )?;
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 5712);
    assert_eq!(handle.height(), 4284);


    let image = lib.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    assert_eq!(image.width(), 5712);
    assert_eq!(image.height(), 4284);

    let img2 = ImageReader::new(image).with_guessed_format()?.decode()?;

    Ok(())
}
