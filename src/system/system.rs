use super::args::Args;
use crate::{error::Error, media::wrapper::Video, terminal::Terminal};
use clap::Parser;

pub struct System {
    args: Args,
}

impl System {
    pub fn new() -> Self {
        let args = Args::parse();
        Self { args }
    }
    fn run_for_image(&self) -> Result<(), Error> {
        let image = crate::media::wrapper::Image::try_from(self.args.file_path.as_path())?;
        Terminal::print_image(image, 1)
    }
    fn run_for_video(&self) -> Result<(), Error> {
        let mut video = Video::try_from(self.args.file_path.as_path())?;
        let frame = video.read_frame()?;
        todo!()
    }
    pub fn run(&self) -> Result<(), Error> {
        if true {
            self.run_for_video()
        } else {
            self.run_for_image()
        }
    }
}
