use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn append_to_file(path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
