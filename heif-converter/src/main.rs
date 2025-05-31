use libheif_rs::{Channel, ColorSpace, HeifContext, ItemId, LibHeif, Result, RgbChroma};
use std::ffi;
use std::ptr;

fn main() -> Result<()>{
    println!("Hello, world!");
    let lib = LibHeif::new();
    let ctx = HeifContext::read_from_file(
        "./data/IMG_1415.HEIC"
    )?;
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 5712);
    assert_eq!(handle.height(), 4284);
    
    Ok(())
}
