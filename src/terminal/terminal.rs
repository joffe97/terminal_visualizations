use crate::{error::Error, image::Image};
use std::cmp::Ordering;
use terminal_size::{terminal_size, Height, Width};

const TERMINAL_BOTTOM_CONTENT_SIZE: u32 = 1;
const TERMINAL_CELL_WIDTH_HEIGHT_RATIO: f32 = 2.5;

struct TerminalImageProperties {
    width: u32,
    height: u32,
    border_width: u32,
    border_height: u32,
}

impl TerminalImageProperties {
    fn new(width: u32, height: u32, border_width: u32, border_height: u32) -> Self {
        Self {
            width,
            height,
            border_width,
            border_height,
        }
    }
}

pub struct Terminal;

impl Terminal {
    fn size() -> Result<(u16, u16), Error> {
        terminal_size()
            .and_then(|(Width(width), Height(height))| Some((width, height)))
            .ok_or(Error::TerminalNotFound)
    }
    fn ratio() -> Result<f32, Error> {
        Self::size().and_then(|(width, height)| Ok(width as f32 / height as f32))
    }
    fn convert_dimensions_to_terminal_dimensions(
        terminal_image_properties: TerminalImageProperties,
        below_content_size: u32,
    ) -> Result<TerminalImageProperties, Error> {
        let TerminalImageProperties {
            width,
            height,
            border_width,
            border_height,
        } = terminal_image_properties;
        let height_without_bottom = height - below_content_size;

        let (terminal_width, terminal_height) = Self::size()?;
        let terminal_width = terminal_width as u32;
        let terminal_height = terminal_height as u32;
        let terminal_height_without_bottom = terminal_height - below_content_size;

        let width_diff_total = terminal_width - width;
        let height_diff_total = terminal_height_without_bottom - height_without_bottom;

        let width_diff_side = width_diff_total / 2;
        let height_diff_side = height_diff_total / 2;

        let border_width_centered = border_width.max(width_diff_side);
        let border_height_centered = border_height.max(height_diff_side);

        let border_width_total = 2 * border_width_centered;
        let border_height_total = 2 * border_height_centered;

        if border_width_total >= terminal_width
            || border_height_total >= terminal_height_without_bottom
        {
            return Err(Error::InvalidDimensions);
        }

        let width_corrected = terminal_width - border_width_total;
        let height_corrected = terminal_height_without_bottom - border_height_total;

        Ok(TerminalImageProperties::new(
            width_corrected,
            height_corrected,
            border_width_centered,
            border_height_centered,
        ))
    }
    fn resize_dimensions_to_terminal_ratio(dimensions_ratio: f32) -> Result<(u32, u32), Error> {
        let (terminal_width, terminal_height) = Self::size()?;

        let terminal_ratio = Self::ratio()?;
        let wanted_terminal_ratio = dimensions_ratio * TERMINAL_CELL_WIDTH_HEIGHT_RATIO;

        match terminal_ratio.partial_cmp(&wanted_terminal_ratio) {
            Some(ordering) => Ok(match ordering {
                Ordering::Greater => (
                    (terminal_height as f32 * wanted_terminal_ratio) as u32,
                    terminal_height.into(),
                ),
                Ordering::Less => (
                    terminal_width.into(),
                    (terminal_width as f32 / wanted_terminal_ratio) as u32,
                ),
                Ordering::Equal => (terminal_width.into(), terminal_height.into()),
            }),
            None => Err(Error::NANError),
        }
    }
    fn get_terminal_image_properties(
        image_ratio: f32,
        border_size: u32,
    ) -> Result<TerminalImageProperties, Error> {
        let (width, height) = Self::resize_dimensions_to_terminal_ratio(image_ratio)?;

        let border_size_width = (border_size as f32 * TERMINAL_CELL_WIDTH_HEIGHT_RATIO) as u32;
        let border_size_height = border_size;

        let terminal_image_properties = TerminalImageProperties::new(
            width.into(),
            height.into(),
            border_size_width,
            border_size_height,
        );

        match Terminal::convert_dimensions_to_terminal_dimensions(
            terminal_image_properties,
            TERMINAL_BOTTOM_CONTENT_SIZE,
        ) {
            Ok(terminal_image_properties) => Ok(terminal_image_properties),
            Err(error @ Error::InvalidDimensions) => {
                if border_size > 0 {
                    Self::get_terminal_image_properties(image_ratio, border_size - 1)
                } else {
                    Err(error)
                }
            }
            Err(error) => Err(error),
        }
    }
    pub fn print_image(image: Image, border_size: u32) -> Result<(), Error> {
        let TerminalImageProperties {
            width,
            height,
            border_width: border_size_width,
            border_height: border_size_height,
        } = Self::get_terminal_image_properties(image.ratio(), border_size)?;

        let ascii_image =
            image.to_ascii_image_with_size(width, height, border_size_width, border_size_height)?;
        print!("{}", ascii_image.to_string());
        Ok(())
    }
}
