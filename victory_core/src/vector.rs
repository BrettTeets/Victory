use crate::angle::Rad;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Vector3{
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3{

    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x, 
            y, 
            z,
        }
    }

    pub fn unit_x() -> Self{
        Self{
            x: 1.0,
            y: 0.0, 
            z: 0.0,
        }
    }

    pub fn unit_y() -> Self{
        Self{
            x: 0.0,
            y: 1.0, 
            z: 0.0,
        }
    }

    pub fn unit_z() -> Self{
        Self{
            x: 0.0,
            y: 0.0, 
            z: 1.0,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn angle_between(&self, other: Vector3) -> Rad  {
        // Note: a and b are normalised within the dotProduct method.
        let product = self.dot(other);
        let rad = product.acos();
        return Rad(rad);
    }

    pub fn dot(&self, other: Vector3) -> f32{
        
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    pub fn cross(&self, other: Vector3) -> Vector3{
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,	
        }
    }
}

impl core::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self {
        Self {
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z, 
        }
    }
}

impl core::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x, 
            y: self.y - other.y, 
            z: self.z - other.z, 
        }
    }
}

impl core::ops::Mul<f32> for Vector3{
    type Output = Self;

    fn mul(self, other: f32) -> Self{
        Self {
            x: self.x * other, 
            y: self.y * other, 
            z: self.z * other, 
        }
    }
}





