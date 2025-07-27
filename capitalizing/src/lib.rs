pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut c = input.chars();
    let first_char = c.next().unwrap().to_uppercase().to_string();
    first_char + c.as_str()
}

pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push_str(&c.to_uppercase().to_string());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect::<String>()
}

// #[cfg(test)]
// mod tests {
//     use super::*;


//     #[test]
//     fn test_capitalize_first() {
//         assert_eq!(capitalize_first("hello"), "Hello");
//         assert_eq!(capitalize_first("this is working"), "This is working");
//     }

//     #[test]
//     fn test_title_case() {
//         assert_eq!(title_case("this is a title"), "This Is A Title");
//         assert_eq!(
//             title_case("hello my\t\tname is carl"),
//             "Hello My\t\tName Is Carl"
//         );
//     }

//     #[test]
//     fn test_change_case() {
//         assert_eq!(change_case("PROgraming"), "proGRAMING");
//         assert_eq!(change_case("heLLo THere"), "HEllO thERE");
//     }

//     #[test]
//     fn test_empty() {
//         assert_eq!(capitalize_first(""), "");
//         assert_eq!(title_case(""), "");
//         assert_eq!(change_case(""), "");
//     }
  
// }
