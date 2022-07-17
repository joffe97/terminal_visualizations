use crate::{
    error::Error,
    media::ascii::{AsciiImage, AsciiTransformer},
};
use image::{imageops::FilterType, io::Reader, DynamicImage};
use std::path::Path;

pub struct Image {
    image: DynamicImage,
}

impl TryFrom<&Path> for Image {
    type Error = crate::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let image = Reader::open(path)?.decode()?;
        Ok(Self::from(image))
    }
}
impl From<DynamicImage> for Image {
    fn from(image: DynamicImage) -> Self {
        Self { image }
    }
}

impl Image {
    pub fn ratio(&self) -> f32 {
        self.image.width() as f32 / self.image.height() as f32
    }
    fn resize_keeping_aspect_ratio(&self, max_width: u32, max_height: u32) -> Self {
        Self::from(
            self.image
                .resize(max_width, max_height, FilterType::Nearest),
        )
    }
    fn resize_discarding_aspect_ratio(&self, width: u32, height: u32) -> Self {
        Self::from(self.image.resize_exact(width, height, FilterType::Nearest))
    }
    fn resize(&self, width: u32, height: u32, keep_aspect_ratio: bool) -> Self {
        if keep_aspect_ratio {
            self.resize_keeping_aspect_ratio(width, height)
        } else {
            self.resize_discarding_aspect_ratio(width, height)
        }
    }
    pub fn to_ascii_image(
        self,
        border_width: u32,
        border_height: u32,
    ) -> Result<AsciiImage, Error> {
        let gray_scale_image = self.image.into_luma8();
        let ascii_pixels = gray_scale_image
            .into_iter()
            .cloned()
            .map(|num| AsciiTransformer::number_to_character(num))
            .collect::<String>();
        AsciiImage::try_new(
            ascii_pixels,
            gray_scale_image.width(),
            gray_scale_image.height(),
            border_width,
            border_height,
        )
    }
    pub fn to_ascii_image_with_size(
        &self,
        width: u32,
        height: u32,
        border_width: u32,
        border_height: u32,
    ) -> Result<AsciiImage, Error> {
        let resized_image = self.resize(width, height, false);
        resized_image.to_ascii_image(border_width, border_height)
    }
}
