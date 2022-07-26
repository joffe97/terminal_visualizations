use crate::error::Error;
use image::{DynamicImage, GrayImage};
use mp4::{read_mp4, Mp4Reader, Mp4Sample};
use std::{borrow::BorrowMut, fs::File, io::BufReader, path::Path};

use super::Image;

pub struct Mp4Video {
    video_reader: Mp4Reader<BufReader<File>>,
}

impl TryFrom<&Path> for Mp4Video {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let video_reader = read_mp4(file)?;
        Ok(Self::from(video_reader))
    }
}

impl From<Mp4Reader<BufReader<File>>> for Mp4Video {
    fn from(video_reader: Mp4Reader<BufReader<File>>) -> Self {
        Self { video_reader }
    }
}

impl Mp4Video {
    pub fn read_frame(&mut self, frame_num: u32) -> Result<Option<Image>, Error> {
        let sample_id = frame_num + 1;
        let (track_id, track) = match self.video_reader.tracks().iter().next() {
            Some((track_id, track)) => (track_id.clone(), track),
            None => return Ok(None),
        };
        let sample_count = track.sample_count();
        if !(1 <= sample_id && sample_id < sample_count) {
            return Err(Error::IndexError);
        }

        let video_profile = track.video_profile()?;

        let width = track.width();
        let height = track.height();
        let sample = match self.video_reader.read_sample(track_id, sample_id)? {
            Some(frame) => frame,
            None => return Ok(None),
        };
        let duration = sample.duration;
        let mut data = sample.bytes.to_vec();
        data.extend(vec![0; (width as usize * height as usize) - data.len()]);
        let gray_image = match GrayImage::from_vec(width.into(), height.into(), data) {
            Some(gray_image) => gray_image,
            None => return Ok(None),
        };
        let dynamic_image = DynamicImage::from(gray_image);
        let image = Image::from(dynamic_image);
        Ok(Some(image))
    }
}
