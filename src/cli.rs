use std::path::PathBuf;
use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    /// Input image
    pub input_image: PathBuf,
    /// Output image
    pub output_image: PathBuf,
    /// Number of threads
    #[arg(
        short = 'j',
        long = "num-threads",
        value_name = "NUM_THREADS",
        default_value_t = 1
    )]
    pub num_threads: usize,
    #[command[subcommand]]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Command {
    /// Adjust the brightness of the image
    Brightness {
        /// Brightness adjustment amount
        amount: f32,
    },
    /// Adjust the contrast of the image
    Contrast {
        /// Contrast adjustment factor
        #[arg(value_parser = valid_contrast)]
        factor: f32, 
    },
    /// Apply gamma correction
    Gamma {
        /// Gamma correction amount
        amount: f32,
    },
    /// Invert
    Invert,
}

fn valid_contrast(s: &str) -> Result<f32, String> {
    let factor: f32 = s.parse().map_err(|_| format!("{s} isn't a float"))?;
    if factor >= 0.0 {
        Ok(factor)
    } else {
        Err(format!("contrast factor is not >= 0.0"))
    }
}