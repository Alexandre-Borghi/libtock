use crate::color::Color;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    value: u8,
    color: Color,
}

impl Card {
    pub const fn new(value: u8, color: Color) -> Self {
        let value = match value {
            value if value >= 1 && value <= 13 => value,
            _ => panic!("Invalid value for card:  not in 1..=13"),
        };

        Self { value, color }
    }
}
