use std::{fs::OpenOptions, io::Write, path::Path};

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let _file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap()
        .write(content.as_bytes())
        .unwrap();
}
