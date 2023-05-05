use std::time::Instant;
use clap::Parser;
use image2::{Image, Rgb};

use image_processor::image_processor::*;
use image_processor::cli::*;

fn main() {
    let program_start = Instant::now();
    // Parse arguments
    let cli = Cli::parse();
    // Construct thread pool
    println!("Using {} theads", cli.num_threads);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(cli.num_threads).build().unwrap();
    // Load image
    println!("Loading image: {:?}", &cli.input_image);
    let mut image = Image::<u8, Rgb>::open(&cli.input_image).unwrap();
    // Process
    let imp_start = Instant::now();
    pool.install(|| process_image(&mut image, cli.command));
    let imp_duration = imp_start.elapsed();
    // Save image
    println!("Saving processed image to: {:?}", &cli.output_image);
    image.save(&cli.output_image).unwrap();
    let program_duration = program_start.elapsed();
    println!("Total elapsed time: {:.2?}ms", program_duration.as_millis());
    println!("Image processing duration: {:.2?}ms", imp_duration.as_millis());
}