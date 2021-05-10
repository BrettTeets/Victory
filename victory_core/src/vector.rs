
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Vector{
    x: f32,
    y: f32,
    z: f32,
}

impl Vector{
    pub const ZERO: Self = Self {x:0.0, y: 0.0, z: 0.0};
    pub const ONE: Self = Self {x:1.0, y: 1.0, z: 1.0};
    pub const X_AXIS: Self = Self {x:1.0, y: 0.0, z: 0.0};
    pub const Y_AXIS: Self = Self {x:0.0, y: 1.0, z: 0.0};
    pub const Z_AXIS: Self = Self {x:0.0, y: 0.0, z: 1.0};
}

