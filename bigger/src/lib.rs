use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut bigger = i32::MIN;
    for (_, value) in h {
        if value > bigger {
            bigger = value;
        }
    }
    bigger
}
#[cfg(test)]
mod tests {
    use super::*;


}
