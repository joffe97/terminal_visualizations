use opencv::{
    core::ToOutputArray,
    types::VectorOfu8,
    videoio::{VideoCapture, VideoCaptureTrait},
};

use crate::error::Error;
use std::path::Path;

pub struct Video {
    video: VideoCapture,
}

impl Drop for Video {
    fn drop(&mut self) {
        self.video.release().unwrap_or_default();
    }
}

impl TryFrom<&Path> for Video {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let path_str = path.to_str().ok_or(Error::StringConversionError)?;
        let video_capture = VideoCapture::from_file(path_str, 0)?;
        Ok(Self::from(video_capture))
    }
}

impl From<VideoCapture> for Video {
    fn from(video_capture: VideoCapture) -> Self {
        Self {
            video: video_capture,
        }
    }
}

impl Video {
    pub fn read_frame(&mut self) -> Result<Option<Vec<u8>>, Error> {
        let mut data = VectorOfu8::new();
        let frame_is_read = self.video.read(&mut data)?;
        Ok(if frame_is_read && !data.is_empty() {
            Some(data.to_vec())
        } else {
            None
        })
    }
}
