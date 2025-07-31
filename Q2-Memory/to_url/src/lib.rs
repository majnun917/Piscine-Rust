pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_url() {
//         assert_eq!(to_url("this is my search"), "this%20is%20my%20search");
//         assert_eq!(to_url("another search "), "another%20search%20");
//     }
// }
