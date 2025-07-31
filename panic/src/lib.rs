use std::fs::File;

pub fn open_file(s: &str) -> File {
    let op_file = File::open(s);
    match op_file {
        Ok(file)=> file,
        Err(e) => panic!("{e:?}")
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
// use std::fs::{self, File};

// #[test]
// #[should_panic(expected = "No such file or directory")]
// fn test_opening() {
//     open_file("file.txt");
// }

// #[test]
// fn test_opening_existing() {
//     let filename = "created.txt";
//     File::create(filename).unwrap();
//     open_file(filename);
//     fs::remove_file(filename).unwrap();
// }
// }
