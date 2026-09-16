#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        let Color { r, g, b, a } = self;
        Color {
            r: swap_value(r, first, second),
            g: swap_value(g, first, second),
            b: swap_value(b, first, second),
            a: swap_value(a, first, second),
        }
    }
}

fn swap_value(value: u8, first: u8, second: u8) -> u8 {
    match value {
        _ if value == first => second,
        _ if value == second => first,
        _ => value,
    }
}