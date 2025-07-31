use std::path::Path;
use std::fs::OpenOptions;
use std::fs;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut options = OpenOptions::new();

    let _file = options.create(true).write(true).append(true).open(path);
    let _ = fs::write(path, content);
    }

#[cfg(test)]
mod tests {

use tempfile::NamedTempFile;



#[test]
fn test_with_file() {
    let mut file = NamedTempFile::new().unwrap();
    let initial_content = "some content\n";
    file.write_all(initial_content.as_bytes()).unwrap();
    let path = file.into_temp_path();
    let content = "hello world!";

    handling::open_or_create(&path, content);

    assert_eq!(
        format!("{}{}", initial_content, content),
        fs::read_to_string(path).unwrap()
    );
}

#[test]
#[should_panic]
fn test_with_file_with_insufficient_permissions() {
    let file = NamedTempFile::new().unwrap();
    let mut permissions = file.as_file().metadata().unwrap().permissions();
    permissions.set_readonly(true);
    file.as_file().set_permissions(permissions).unwrap();
    let path = file.into_temp_path();
    let content = "hello world!";

    handling::open_or_create(&path, content);
}
}
