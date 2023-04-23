use std::path::PathBuf;
use std::thread;

use clap::{Parser,Subcommand};
use image::io::Reader as ImageReader;

use image_processor as imp;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Input image
    input_image: PathBuf,
    /// Output image
    output_image: PathBuf,
    /// Number of threads
    #[arg(short = 'j', long = "num-threads", value_name = "NUM_THREADS", default_value_t = 1)]
    num_threads: u8,
    #[command[subcommand]]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adjust the exposure of the image
    Exposure { 
        /// Exposure adjustment amount
        amount: f64 
    },
}

fn main() {
    let cli = Cli::parse();
    dbg!(&cli);
    // Load image
    let rgb_img = ImageReader::open(&cli.input_image).unwrap().decode().unwrap().into_rgb8();
    println!("Done");
}
