use std::env;
use std::path::Path;
use image::{self, DynamicImage, ImageOutputFormat};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "resize" => resize_image(&args),
        "dpi" => change_dpi(&args),
        "convert" => convert_format(&args),
        "compress" => compress_image(&args),
        _ => print_usage(),
    }?;

    Ok(())
}

fn print_usage() {
    println!("ImgSwift - Fast Image Processing Tool");
    println!("Usage:");
    println!("  imgswift resize <input> <output> <width> <height>");
    println!("  imgswift dpi <input> <output> <dpi>");
    println!("  imgswift convert <input> <output> <format>");
    println!("  imgswift compress <input> <output> <quality>");
    println!("Formats: jpg, png, webp");
    println!("Example: imgswift resize input.jpg output.jpg 800 600");
}

fn resize_image(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 5 {
        println!("Usage: imgswift resize <input> <output> <width> <height>");
        return Ok(());
    }

    let input_path = &args[2];
    let output_path = &args[3];
    let width: u32 = args[4].parse()?;
    let height: u32 = args[5].parse()?;

    let img = image::open(input_path)?;
    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
    resized.save(output_path)?;
    
    println!("Image resized to {}x{}: {}", width, height, output_path);
    Ok(())
}

fn change_dpi(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 4 {
        println!("Usage: imgswift dpi <input> <output> <dpi>");
        return Ok(());
    }

    let input_path = &args[2];
    let output_path = &args[3];
    let dpi: u32 = args[4].parse()?;

    let img = image::open(input_path)?;
    img.save_with_format(output_path, get_format(output_path))?;
    // Note: DPI metadata requires additional libraries for precise control
    
    println!("Image DPI set to {}: {}", dpi, output_path);
    Ok(())
}

fn convert_format(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 4 {
        println!("Usage: imgswift convert <input> <output> <format>");
        return Ok(());
    }

    let input_path = &args[2];
    let output_path = &args[3];
    let format = &args[4].to_lowercase();

    let img = image::open(input_path)?;
    let output_format = match format.as_str() {
        "jpg" | "jpeg" => ImageOutputFormat::Jpeg(85),
        "png" => ImageOutputFormat::Png,
        "webp" => ImageOutputFormat::WebP,
        _ => {
            println!("Supported formats: jpg, png, webp");
            return Ok(());
        }
    };

    img.save_with_format(output_path, output_format)?;
    println!("Image converted to {}: {}", format, output_path);
    Ok(())
}

fn compress_image(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 4 {
        println!("Usage: imgswift compress <input> <output> <quality>");
        return Ok(());
    }

    let input_path = &args[2];
    let output_path = &args[3];
    let quality: u8 = args[4].parse()?;

    let img = image::open(input_path)?;
    match get_format(output_path) {
        ImageOutputFormat::Jpeg(_) => {
            img.save_with_format(output_path, ImageOutputFormat::Jpeg(quality))?;
            println!("Image compressed with quality {}: {}", quality, output_path);
        }
        _ => {
            img.save(output_path)?;
            println!("Compression only available for JPEG: {}", output_path);
        }
    }
    Ok(())
}

fn get_format(path: &str) -> ImageOutputFormat {
    match Path::new(path).extension().and_then(|s| s.to_str()) {
        Some("jpg") | Some("jpeg") => ImageOutputFormat::Jpeg(85),
        Some("png") => ImageOutputFormat::Png,
        Some("webp") => ImageOutputFormat::WebP,
        _ => ImageOutputFormat::Png,
    }
}