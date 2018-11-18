use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn open_file(filename: &str) -> io::Result<Vec<u8>> {
    let file = File::open(&filename);
    let mut contents: Vec<u8> = Vec::new();
    match file {
        Ok(mut t) => {
            //t.lock_exclusive()?;
            t.read_to_end(&mut contents).unwrap();
            //t.unlock()?;
            Ok(contents)
        }
        Err(e) => {
            println!("{:?}{}", "Unable to open file ", filename);
            Err(e)
        }
    }
}
