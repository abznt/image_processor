use image2::*;
use crate::cli::Command;

/// Apply an image processing algorithm using 1 or more threads
pub fn process_image(image: &mut Image<u8, Rgb>, command: Command) {
    match command {
        Command::Brightness { amount } => {
            image.iter_mut().for_each(|(_, pixel)| {
                adjust_brightness(pixel, amount);
            })
        },
        Command::Contrast { factor } => {
            image.iter_mut().for_each(|(_, pixel)| {
                adjust_contrast(pixel, factor);
            })
        },
        Command::Gamma { amount } => {
            image.iter_mut().for_each(|(_, pixel)| {
                gamma_correction(pixel, amount);
            })
        },
        Command::Invert => {
            image.iter_mut().for_each(|(_, pixel)| {
                invert(pixel);
            })
        }
    }
}

/// Applies brightness adjustment to a single pixel
pub fn adjust_brightness(mut pixel: DataMut<u8, Rgb>, amount: f32) {
    pixel[0] = truncate_to_u8(pixel[0] as f32 + amount);
    pixel[1] = truncate_to_u8(pixel[1] as f32 + amount);
    pixel[2] = truncate_to_u8(pixel[2] as f32 + amount);
}

/// Applies contrast adjustment to a single pixel
pub fn adjust_contrast(mut pixel: DataMut<u8, Rgb>, factor: f32) {
    pixel[0] = truncate_to_u8(factor * (pixel[0] as f32 - 128.0) + 128.0);
    pixel[1] = truncate_to_u8(factor * (pixel[1] as f32 - 128.0) + 128.0);
    pixel[2] = truncate_to_u8(factor * (pixel[2] as f32 - 128.0) + 128.0);
}

/// Applies gamma correction to a single pixel
pub fn gamma_correction(mut pixel: DataMut<u8, Rgb>, amount: f32) {
    let correction = 1.0 / amount;
    pixel[0] = truncate_to_u8(255.0 * (pixel[0] as f32 / 255.0).powf(correction));
    pixel[1] = truncate_to_u8(255.0 * (pixel[1] as f32 / 255.0).powf(correction));
    pixel[2] = truncate_to_u8(255.0 * (pixel[2] as f32 / 255.0).powf(correction));
}

/// Inverts a single pixel
pub fn invert(mut pixel: DataMut<u8, Rgb>) {
    pixel[0] = 255u8 - pixel[0];
    pixel[1] = 255u8 - pixel[1];
    pixel[2] = 255u8 - pixel[2];
}

/// Clamps an f32 to a valid u8
fn truncate_to_u8(f: f32) -> u8 {
    match f {
        x if x <= 0.0 => 0u8,
        x if x >= 255.0 => 255u8,
        _ => f as u8,
    }
}