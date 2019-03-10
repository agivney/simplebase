use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

//extern crate fs2;
//use file_services::fs2::FileExt;
//use engine::fs2::FileExt;

extern crate fs2;
use self::fs2::FileExt;


pub fn obfuscate_data(data_to_obfuscate: String) -> String {
    let string_to_vector = &data_to_obfuscate.as_bytes();
    let mut new_obfuscated_vector: Vec<u8> = Vec::new();
    let obfuscation_vector = vec![0x34, 0xc5, 0xd4, 0x54, 0x00, 0xd3, 0x2a, 0x55];
    let mut counter = 0;
    for i in string_to_vector.iter() {
        new_obfuscated_vector.push(i ^ obfuscation_vector[counter]);
        if counter > obfuscation_vector.len() - 1 {
            counter = 0;
        }
    }

    let result = String::from_utf8_lossy(&new_obfuscated_vector);
    result.to_string()
}
pub fn open_file(filename: &str) -> io::Result<Vec<u8>> {
    let file = File::open(&filename);
    let mut contents: Vec<u8> = Vec::new();
    match file {
        Ok(mut t) => {
            t.lock_exclusive()?;
            t.read_to_end(&mut contents).unwrap();
            t.unlock()?;
            Ok(contents)

        //     loop {
        //     match t.lock_exclusive() {
        //         Ok(_) => {
        //             file.read_to_end(&mut contents).unwrap();
        //             file.unlock().unwrap();
        //             return Ok(contents);
        //         }
        //         Err(_) => (),
        //     }
        // }
        }
        Err(e) => {
            println!("{:?}{}", "Unable to open file ", filename);
            Err(e)
        }
    }
}

pub fn save_data(filename: &str, buf: &[u8]){

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .unwrap();

    loop {
            match file.lock_exclusive() {
                Ok(_) => {
                    file.write_all(buf)
                        .unwrap();
                    file.unlock().unwrap();
                    return;
                }
                Err(_) => (),
            }
        }




}
