use super::args::Args;
use crate::{error::Error, image::Image, terminal::Terminal};
use clap::Parser;

pub struct System {
    args: Args,
}

impl System {
    pub fn from_args() -> Self {
        let args = Args::parse();
        Self { args }
    }
    pub fn run(&self) -> Result<(), Error> {
        let image = Image::try_from(self.args.file_path.as_path())?;
        Terminal::print_image(image, 1)
    }
}
