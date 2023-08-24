use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
};

// output something into file
// this function is used to debug.
pub fn log(content: &str) -> io::Result<()> {
    File::create("log.txt")?;
    let mut file = OpenOptions::new().write(true).open("log.txt")?;
    file.write(content.as_bytes())?;
    file.flush()?;
    Ok(())
}
