pub struct TerminalImageProperties {
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub border_height: u32,
}

impl TerminalImageProperties {
    pub fn new(width: u32, height: u32, border_width: u32, border_height: u32) -> Self {
        Self {
            width,
            height,
            border_width,
            border_height,
        }
    }
}
