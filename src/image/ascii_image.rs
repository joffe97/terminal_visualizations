use crate::error::Error;

pub struct AsciiImage {
    content: String,
    width: u32,
    height: u32,
}

impl AsciiImage {
    pub fn try_new(content: String, width: u32, height: u32) -> Result<Self, Error> {
        if content.len() != (width * height) as usize {
            return Err(Error::DifferentDimensions);
        }
        Ok(Self {
            content,
            width,
            height,
        })
    }
    pub fn print(&self, border_sizes: (u32, u32)) {
        let content_iterator = &mut self
            .content
            .as_bytes()
            .into_iter()
            .map(|byte| char::from(*byte));
        let (border_size_width, border_size_height) = border_sizes;
        let border_string_horizontal = " ".repeat(border_size_width as usize);
        let border_string_verical = " ".repeat((self.width + 2 * border_size_height) as usize);

        println!("{}", border_string_verical);
        for _ in 0..self.height {
            let line_string = content_iterator
                .take(self.width as usize)
                .collect::<String>();
            println!(
                "{}{}{}",
                border_string_horizontal, line_string, border_string_horizontal
            );
        }
        println!("{}", border_string_verical);
    }
}
