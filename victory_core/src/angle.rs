use crate::basic::{FloatPoint};
use num_traits::{cast};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Rad<T>(pub T);


#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Deg<T>(pub T);


impl<T> From<Rad<T>> for Deg<T> where T: FloatPoint,
{
    //could be inlined?
    fn from(rad: Rad<T>) -> Deg<T> {
        Deg(rad.0 * cast(180.0 / std::f64::consts::PI).unwrap())
    }
}

impl<T> From<Deg<T>> for Rad<T> where T: FloatPoint,
{
    //could be inlined?
    fn from(deg: Deg<T>) -> Rad<T> {
        Rad(deg.0 * cast(std::f64::consts::PI / 180.0).unwrap())
    }
}