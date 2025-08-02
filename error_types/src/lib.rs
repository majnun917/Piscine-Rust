use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FormError {
    form_values: HashMap<String, String>,
    #[allow(dead_code)]
    date: String,
    err: &'static str,
}


impl PartialEq for FormError {
    fn eq(&self, other: &Self) -> bool {
        self.form_values == other.form_values && self.err == other.err
    }
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let mut form_values = HashMap::new();
        form_values.insert(field_name.to_string(), field_value);
        FormError {
            form_values,
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        let have_let = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let have_num = self.password.chars().any(|c| c.is_ascii_digit());
        let have_symb = self.password.chars().any(|c| c.is_ascii_punctuation());

        if !have_let || !have_num || !have_symb {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_validate_ok() {
//         let form = Form {
//             name: "John".to_string(),
//             password: "asDd123=%".to_string(),
//         };
//         assert_eq!(form.validate(), Ok(()));
//     }

//     #[test]
//     fn test_validate_empty_name() {
//         let form = Form {
//             name: "".to_string(),
//             password: "asDd123=%".to_string(),
//         };
//         let expected_error = FormError::new("name", "".to_string(), "Username is empty");
//         assert_eq!(form.validate(), Err(expected_error));
//     }

//     #[test]
//     fn test_validate_password_too_short() {
//         let form = Form {
//             name: "John".to_string(),
//             password: "aB1$".to_string(),
//         };
//         let expected_error = FormError::new(
//             "password",
//             "aB1$".to_string(),
//             "Password should be at least 8 characters long",
//         );
//         assert_eq!(form.validate(), Err(expected_error));
//     }

//     #[test]
//     fn test_validate_password_missing_chars() {
//         let passwords = vec!["asgfD123", "asgfD_!@", "12345_!@"];
//         for pwd in passwords {
//             let form = Form { name: "John".to_string(), password: pwd.to_string() };
//             let expected_error = FormError::new(
//                 "password",
//                 pwd.to_string(),
//                 "Password should be a combination of ASCII numbers, letters and symbols",
//             );
//             assert_eq!(form.validate(), Err(expected_error));
//         }
//     }
// }
