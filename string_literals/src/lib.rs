pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//      #[test]
//     fn test_functions_and_memory_allocation() {
//         //ALLOCATOR.reset_counter();

//         assert!(is_empty(""));
//         assert!(!is_empty("something"));
//         assert!(is_ascii("rust"));
//         assert!(!is_ascii("rust¬"));
//         assert!(contains("rust", "ru"));
//         assert!(!contains("something", "mer"));
//         assert_eq!(split_at("rust", 2), ("ru", "st"));
//         assert_eq!(find("ru-st-e", '-'), 2);
//         assert_eq!(find("ru-st-e", 'e'), 6);

//         //assert_eq!(ALLOCATOR.counter(), 0);
//     }
// }
