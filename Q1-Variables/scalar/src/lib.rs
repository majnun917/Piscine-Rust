pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pro(6, 2);
        assert_eq!(result, 12);
         let result = rem(-128.23, 2.0);
        assert_eq!(result,-0.22999573);
        let result = quo(22.0, 2.0);
        assert_eq!(result, 11);
    }
}
