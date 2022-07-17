use super::{
    terminal_image_properties::TerminalImageProperties, terminal_instance::TerminalInstance,
};
use crate::{error::Error, media::wrapper::Image};

pub struct Terminal;

impl Terminal {
    pub fn print_image(image: Image, border_size: u32) -> Result<(), Error> {
        let TerminalImageProperties {
            width,
            height,
            border_width: border_size_width,
            border_height: border_size_height,
        } = TerminalInstance::try_new()?
            .get_terminal_image_properties(image.ratio(), border_size)?;

        let ascii_image =
            image.to_ascii_image_with_size(width, height, border_size_width, border_size_height)?;
        print!("{}", ascii_image.to_string());
        Ok(())
    }
}
