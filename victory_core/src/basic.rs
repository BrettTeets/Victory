use std::fmt;
use std::ops::*;
use num_traits::{Float, Num, NumCast}; //I wonder if I should try to do this stuff myself as well?
use approx; //What even is this?

//Integers I believe.
pub trait Number: Copy + Clone + fmt::Debug + Num + NumCast + PartialOrd 
+ AddAssign + SubAssign + MulAssign + DivAssign + RemAssign {}

impl<T> Number for T where T: Copy + Clone + fmt::Debug + Num + NumCast + PartialOrd 
+ AddAssign + SubAssign + MulAssign + DivAssign + RemAssign {}

//floating point numbers.
pub trait FloatPoint: Number + Float + approx::AbsDiffEq<Epsilon = Self> 
+ approx::RelativeEq<Epsilon = Self> + approx::UlpsEq<Epsilon = Self> {}

impl<T> FloatPoint for T where T: Number + Float + approx::AbsDiffEq<Epsilon = Self>
+ approx::RelativeEq<Epsilon = Self> + approx::UlpsEq<Epsilon = Self> {}