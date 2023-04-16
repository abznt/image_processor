use std::path::PathBuf;
use clap::{Parser,Subcommand};

#[derive(Parser)]
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

#[derive(Subcommand)]
enum Commands {
    /// Adjust the exposure of the image
    Exposure { 
        /// Exposure adjustment amount
        amount: f64 
    },
}

fn main() {
    let cli = Cli::parse();
}
