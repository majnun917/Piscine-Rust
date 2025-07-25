pub fn str_len(s: &str) -> usize {
    s.chars().count()
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//      #[test]
//     fn passes() {
//         let inputs = [
//             ("hello", 5),
//             ("how", 3),
//             ("are you", 7),
//             ("change", 6),
//             ("olá!", 4),
//             ("bitteschön", 10),
//         ];

//         inputs
//             .into_iter()
//             .for_each(|(s, l)| assert_eq!(l, str_len(s)));
//     }
// }
