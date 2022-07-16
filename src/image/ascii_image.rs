use crate::error::Error;

pub struct AsciiImage {
    content: String,
    content_width: u32,
    content_height: u32,
    border_width: u32,
    border_height: u32,
}

impl ToString for AsciiImage {
    fn to_string(&self) -> String {
        let mut final_string = String::with_capacity(self.total_size_with_newlines());

        let content_iterator = &mut self
            .content
            .as_bytes()
            .into_iter()
            .map(|byte| char::from(*byte));

        let border_string_horizontal = " ".repeat(self.border_width as usize);
        let border_string_verical = " "
            .repeat(((self.content_width + 2 * self.border_width) * self.border_height) as usize);

        final_string.push_str(&border_string_verical);
        final_string.push_str("\n");
        for _ in 0..self.content_height {
            let line_string = content_iterator
                .take(self.content_width as usize)
                .collect::<String>();
            final_string.push_str(&border_string_horizontal);
            final_string.push_str(&line_string);
            final_string.push_str(&border_string_horizontal);
            final_string.push_str("\n");
        }
        final_string.push_str(&border_string_verical);
        final_string.push_str("\n");

        final_string
    }
}

impl AsciiImage {
    pub fn try_new(
        content: String,
        width: u32,
        height: u32,
        border_width: u32,
        border_height: u32,
    ) -> Result<Self, Error> {
        if content.len() != (width * height) as usize {
            return Err(Error::DifferentDimensions);
        }
        Ok(Self {
            content,
            content_width: width,
            content_height: height,
            border_width,
            border_height,
        })
    }
    fn total_width(&self) -> usize {
        (self.content_width + self.border_width) as usize
    }
    fn total_height(&self) -> usize {
        (self.content_height + self.border_height) as usize
    }
    fn total_size(&self) -> usize {
        self.total_width() * self.total_height()
    }
    fn total_size_with_newlines(&self) -> usize {
        self.total_size() + self.total_height()
    }
}
