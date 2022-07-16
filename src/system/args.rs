use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(default_value = r"/home/joachim/Pictures/intersection_now.png")]
    pub file_path: PathBuf,

    #[clap(default_value_t = 150)]
    pub width: u32,

    #[clap(default_value_t = 100)]
    pub height: u32,
}
