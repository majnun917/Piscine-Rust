pub fn rev_str(input: &str) -> String {
    let mut rev_str = String::new();
    for c in input.chars().rev() {
        rev_str.push(c);
    }
    rev_str
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rev_str("Hello, my name is Roman");
        assert_eq!(result, "namoR si eman ym ,olleH");
    }
}
