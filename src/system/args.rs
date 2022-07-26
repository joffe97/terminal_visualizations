use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    // #[clap(default_value = r"/home/joachim/Pictures/intersection_now.png")]
    #[clap(default_value = r"/home/joachim/Downloads/A0485_F1507_P2_Pink_Comp_1.mp4")]
    pub file_path: PathBuf,

    #[clap(default_value_t = 1)]
    pub border: u32,
}
