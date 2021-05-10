use std::f32::consts::PI as PI;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Rad(f32);

impl Rad{
    pub const ONEPI: Self = Rad(PI); //180 degrees
    pub const TWOPI: Self = Rad(2.0*PI); //360 degrees
    pub const THIRDPI: Self = Rad((1.0/3.0)*PI); //60 degrees
}




