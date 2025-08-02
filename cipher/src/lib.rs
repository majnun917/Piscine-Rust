#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = decode(original);
    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}

pub fn decode(text: &str) -> String {
    text
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let offset = c as u8 - b'a';
                let atbash = b'z' - offset;
                char::from(atbash)
            } else if c.is_ascii_uppercase() {
                let offset = c as u8 - b'A';
                let atbash = b'Z' - offset;
                char::from(atbash)
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
   use cipher::*;

#[test]
fn test_ok_values() {
    assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Ok(()));
    assert_eq!(cipher("asdasd", "zhwzhw"), Ok(()));
    assert_eq!(cipher("3(/&%fsd 32das", "3(/&%uhw 32wzh"), Ok(()));
}

#[test]
fn test_empty_values() {
    assert_eq!(cipher("", ""), Ok(()));
    assert_eq!(
        cipher("", "1Svool 2dliow!"),
        Err(CipherError {
            expected: "".to_owned()
        })
    );
    assert_eq!(
        cipher("1Hello 2world!", ""),
        Err(CipherError {
            expected: "1Svool 2dliow!".to_owned()
        })
    );
}

#[test]
fn test_errors() {
    assert_eq!(
        cipher("1Hello 2world!", "1svool 2dliow!"),
        Err(CipherError {
            expected: String::from("1Svool 2dliow!")
        })
    );
    assert_eq!(
        cipher("asdasd", "lkdas"),
        Err(CipherError {
            expected: String::from("zhwzhw")
        })
    );
    assert_eq!(
        cipher("3(/&%sd 32das", "3(/&%uhw 32wzh"),
        Err(CipherError {
            expected: String::from("3(/&%hw 32wzh")
        })
    );
}
}
