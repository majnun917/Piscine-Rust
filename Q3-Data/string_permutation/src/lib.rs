pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.chars().count() != s2.chars().count() {
        return false;
    }
    let mut s1_chars: Vec<char> = s1.chars().collect();
    let mut s2_chars: Vec<char> = s2.chars().collect();

    s1_chars.sort_unstable();
    s2_chars.sort_unstable();

    s1_chars == s2_chars
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_basic() {
//         assert!(is_permutation("abcde", "edbca"));
//         assert!(!is_permutation("avcde", "edbca"));
//         assert!(!is_permutation("cde", "edbca"));
//         assert!(is_permutation("code", "deco"));
//         assert!(!is_permutation("code", "deeco"));
//         assert!(!is_permutation("codde", "deeco"));
//     }

//     #[test]
//     fn test_repeating_characters() {
//         assert!(is_permutation("aab", "baa"));
//     }

//     #[test]
//     fn test_one_char() {
//         assert!(!is_permutation("a", "b"));
//         assert!(is_permutation("a", "a"));
//     }

//     #[test]
//     fn test_empty() {
//         assert!(is_permutation("", ""));
//     }

//     #[test]
//     fn test_special_characters() {
//         assert!(is_permutation("!#%@", "@%#!"));
//     }

//     #[test]
//     fn test_unicode() {
//         assert!(is_permutation("hello♥", "♥oelhl"));
//         assert!(!is_permutation("♥", "♥♥"));
//     }
// }