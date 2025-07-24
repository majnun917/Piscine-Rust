pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: i32, b: i32) -> i32 {
    a / b
}

pub fn rem(a: i32, b: i32) -> i32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pro(6, 2);
        assert_eq!(result, 12);
         let result = rem(6, 2);
        assert_eq!(result, 0);
    }
}
