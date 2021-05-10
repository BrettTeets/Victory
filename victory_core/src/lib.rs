pub mod basic;
pub mod angle;


pub struct Vector3<T>{
    x: T,
    y: T,
    z: T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


