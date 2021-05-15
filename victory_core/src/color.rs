use std::mem;

pub struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

//Constant Colors
impl Color {
    pub const RED: Color = Color {r: 255, g: 0, b: 0, a: 255};
    pub const GREEN: Color = Color {r: 0, g: 255, b: 0, a: 255};
    pub const BLUE: Color = Color {r: 0, g: 0, b: 255, a: 255};
}

impl From<Color> for u32 {
    fn from(item: Color) -> Self {
        unsafe { mem::transmute::<[u8; 4], u32>([item.r, item.g, item.b, item.a]) }
    }
}
