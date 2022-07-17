use crate::error::Error;
use std::cmp::Ordering;
use terminal_size::{terminal_size, Height, Width};

use super::terminal_image_properties::TerminalImageProperties;

const TERMINAL_BOTTOM_CONTENT_SIZE: u32 = 1;
const TERMINAL_CELL_WIDTH_HEIGHT_RATIO: f32 = 2.5;

pub struct TerminalInstance {
    width: u32,
    height: u32,
}

impl TerminalInstance {
    pub fn try_new() -> Result<Self, Error> {
        terminal_size()
            .and_then(|(Width(width), Height(height))| {
                Some(Self {
                    width: width.into(),
                    height: height.into(),
                })
            })
            .ok_or(Error::TerminalNotFound)
    }
    fn ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    fn convert_dimensions_to_terminal_dimensions(
        &self,
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

        let terminal_height_without_bottom = self.height - below_content_size;

        let width_diff_total = self.width - width;
        let height_diff_total = terminal_height_without_bottom - height_without_bottom;

        let width_diff_side = width_diff_total / 2;
        let height_diff_side = height_diff_total / 2;

        let border_width_centered = border_width.max(width_diff_side);
        let border_height_centered = border_height.max(height_diff_side);

        let border_width_total = 2 * border_width_centered;
        let border_height_total = 2 * border_height_centered;

        if border_width_total >= self.width || border_height_total >= terminal_height_without_bottom
        {
            return Err(Error::InvalidDimensions);
        }

        let width_corrected = self.width - border_width_total;
        let height_corrected = terminal_height_without_bottom - border_height_total;

        Ok(TerminalImageProperties::new(
            width_corrected,
            height_corrected,
            border_width_centered,
            border_height_centered,
        ))
    }
    fn resize_dimensions_to_terminal_ratio(
        &self,
        dimensions_ratio: f32,
    ) -> Result<(u32, u32), Error> {
        let terminal_ratio = self.ratio();
        let wanted_terminal_ratio = dimensions_ratio * TERMINAL_CELL_WIDTH_HEIGHT_RATIO;

        match terminal_ratio.partial_cmp(&wanted_terminal_ratio) {
            Some(ordering) => Ok(match ordering {
                Ordering::Greater => (
                    (self.height as f32 * wanted_terminal_ratio) as u32,
                    self.height,
                ),
                Ordering::Less => (
                    self.width,
                    (self.width as f32 / wanted_terminal_ratio) as u32,
                ),
                Ordering::Equal => (self.width, self.height),
            }),
            None => Err(Error::NANError),
        }
    }
    pub fn get_terminal_image_properties(
        &self,
        image_ratio: f32,
        border_size: u32,
    ) -> Result<TerminalImageProperties, Error> {
        let (width, height) = self.resize_dimensions_to_terminal_ratio(image_ratio)?;

        let border_size_width = (border_size as f32 * TERMINAL_CELL_WIDTH_HEIGHT_RATIO) as u32;
        let border_size_height = border_size;

        let terminal_image_properties = TerminalImageProperties::new(
            width.into(),
            height.into(),
            border_size_width,
            border_size_height,
        );

        match self.convert_dimensions_to_terminal_dimensions(
            terminal_image_properties,
            TERMINAL_BOTTOM_CONTENT_SIZE,
        ) {
            Ok(terminal_image_properties) => Ok(terminal_image_properties),
            Err(error @ Error::InvalidDimensions) => {
                if border_size > 0 {
                    self.get_terminal_image_properties(image_ratio, border_size - 1)
                } else {
                    Err(error)
                }
            }
            Err(error) => Err(error),
        }
    }
}
