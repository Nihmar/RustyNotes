use std::fs;
use std::io;
use std::path::Path;

pub fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

pub fn file_exists(path: &Path) -> bool {
    path.exists()
}
