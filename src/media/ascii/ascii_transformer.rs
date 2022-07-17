use num_traits::{AsPrimitive, Bounded, Num};

const ASCII_CHARACTERS: &str = ".,-~:;=!*#$@";

pub struct AsciiTransformer;

impl AsciiTransformer {
    fn get_char_at_index(index: usize) -> char {
        char::from(ASCII_CHARACTERS.as_bytes()[index.clamp(0, ASCII_CHARACTERS.len() - 1)])
    }
    pub fn number_to_character<T>(num: T) -> char
    where
        T: Num + Copy + AsPrimitive<f32> + Bounded,
    {
        let min_value = T::min_value();
        let max_value = T::max_value();
        let rate = (num.as_() - min_value.as_()) / max_value.as_();
        let index = (rate * ASCII_CHARACTERS.len() as f32).ceil() - 1.0;
        Self::get_char_at_index(index as usize)
    }
}

#[test]
fn test255() {
    let left = AsciiTransformer::number_to_character(255 as u8);
    let right = char::from("@".as_bytes()[0]);
    assert_eq!(left, right);
}
#[test]
fn test254() {
    let left = AsciiTransformer::number_to_character(254 as u8);
    let right = char::from("@".as_bytes()[0]);
    assert_eq!(left, right);
}
#[test]
fn test0() {
    let left = AsciiTransformer::number_to_character(0 as u8);
    let right = char::from(".".as_bytes()[0]);
    assert_eq!(left, right);
}
#[test]
fn test1() {
    let left = AsciiTransformer::number_to_character(1 as u8);
    let right = char::from(".".as_bytes()[0]);
    assert_eq!(left, right);
}
#[test]
fn test_f32_max() {
    let left = AsciiTransformer::number_to_character(f32::MAX);
    let right = char::from("@".as_bytes()[0]);
    assert_eq!(left, right);
}
#[test]
fn test_f32_min() {
    let left = AsciiTransformer::number_to_character(f32::MIN);
    let right = char::from(".".as_bytes()[0]);
    assert_eq!(left, right);
}
