pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|init| {
            init.split_whitespace()
                .map(|part| format!("{}.", part.chars().next().unwrap()))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//      #[test]
//     fn test_function() {
//         let cases = [
//             (
//                 vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"],
//                 vec!["H. P.", "S. E.", "J. L.", "B. O."],
//             ),
//             (
//                 vec![
//                     "James John",
//                     "David Joseph",
//                     "Matthew Brian",
//                     "Jacob Sousa",
//                     "Bruce Banner",
//                     "Scarlett Johansson",
//                     "Graydon Hoare",
//                 ],
//                 vec![
//                     "J. J.", "D. J.", "M. B.", "J. S.", "B. B.", "S. J.", "G. H.",
//                 ],
//             ),
//         ];

//         cases
//             .into_iter()
//             .for_each(|(n, e)| assert_eq!(initials(n), e));
//     }
// }
