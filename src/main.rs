use image::{GenericImageView, ImageBuffer, Rgba};
use std::env;
use std::path::Path;

/// Compute the greatest common divisor (GCD) of two numbers.
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Given an image, scan each row and column to compute the GCD
/// of run lengths of identical pixels. For a perfect pixel art image,
/// this GCD should be the block size.
fn detect_block_size(img: &image::DynamicImage) -> Option<u32> {
    let (width, height) = img.dimensions();

    let mut gcd_row: Option<u32> = None;
    for y in 0..height {
        let mut last_color = img.get_pixel(0, y);
        let mut last_change = 0;
        for x in 1..width {
            let color = img.get_pixel(x, y);
            if color != last_color {
                let run_length = x - last_change;
                gcd_row = Some(match gcd_row {
                    None => run_length,
                    Some(g) => gcd(g, run_length),
                });
                last_change = x;
                last_color = color;
            }
        }
        let run_length = width - last_change;
        gcd_row = Some(match gcd_row {
            None => run_length,
            Some(g) => gcd(g, run_length),
        });
    }

    let mut gcd_col: Option<u32> = None;
    for x in 0..width {
        let mut last_color = img.get_pixel(x, 0);
        let mut last_change = 0;
        for y in 1..height {
            let color = img.get_pixel(x, y);
            if color != last_color {
                let run_length = y - last_change;
                gcd_col = Some(match gcd_col {
                    None => run_length,
                    Some(g) => gcd(g, run_length),
                });
                last_change = y;
                last_color = color;
            }
        }
        let run_length = height - last_change;
        gcd_col = Some(match gcd_col {
            None => run_length,
            Some(g) => gcd(g, run_length),
        });
    }

    if let (Some(gr), Some(gc)) = (gcd_row, gcd_col) {
        Some(gcd(gr, gc))
    } else {
        None
    }
}

/// Downsamples the image by taking one pixel for each block of size `block_size`.
fn downsample_image(img: &image::DynamicImage, block_size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let new_width = width / block_size;
    let new_height = height / block_size;

    let mut new_img = ImageBuffer::new(new_width, new_height);

    for j in 0..new_height {
        for i in 0..new_width {
            let pixel = img.get_pixel(i * block_size, j * block_size);
            new_img.put_pixel(i, j, pixel);
        }
    }
    new_img
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_image> <output_image>", args[0]);
        return;
    }
    let input_path = &args[1];
    let output_path = &args[2];

    let img = match image::open(&Path::new(input_path)) {
        Ok(im) => im,
        Err(e) => {
            eprintln!("Error opening image {}: {}", input_path, e);
            return;
        }
    };

    let block_size = match detect_block_size(&img) {
        Some(size) => size,
        None => {
            eprintln!("Could not determine block size.");
            return;
        }
    };

    println!("Detected block size: {}x{}", block_size, block_size);

    let fixed_img = downsample_image(&img, block_size);

    if let Err(e) = fixed_img.save(&Path::new(output_path)) {
        eprintln!("Error saving image {}: {}", output_path, e);
    } else {
        println!("Saved fixed image to {}", output_path);
    }
}
