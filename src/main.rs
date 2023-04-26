use std::thread;
use std::{path::PathBuf, sync::Arc};

use clap::{Parser, Subcommand};
use image::{io::Reader as ImageReader, GenericImageView};

use image_processor as imp;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Input image
    input_image: PathBuf,
    /// Output image
    output_image: PathBuf,
    /// Number of threads
    #[arg(
        short = 'j',
        long = "num-threads",
        value_name = "NUM_THREADS",
        default_value_t = 1
    )]
    num_threads: u32,
    #[command[subcommand]]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adjust the exposure of the image
    Exposure {
        /// Exposure adjustment amount
        amount: f32,
    },
}

fn main() {
    let cli = Cli::parse();
    dbg!(&cli);
    // Load image
    let mut rgb_img = 
        ImageReader::open(&cli.input_image)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8()
    ;

    thread::scope(|s| {
        for partition in imp::get_partitions(rgb_img.width(), rgb_img.height(), cli.num_threads) {
            // let cli = cli.clone();
            // let rgb_img = rgb_img.clone();
            s.spawn(|| {
                for x in partition.xmin..partition.xmax {
                    for y in partition.ymin..partition.ymax {
                        match cli.command {
                            Commands::Exposure { amount } => {
                                imp::adjust_exposure(rgb_img.get_pixel_mut(x, y), amount);
                            }
                        }
                    }
                }
            });
        }
    });
    println!("Done");
}
