use std::fs::File;
use std::io::{Read, Write, Result};
use std::path::Path;

pub(super) fn read(p: &String) -> Result<String> {
    // add sanitization to the path argument
    let path = Path::new(&p);
    let mut file = File::options().read(true).write(true)
        .create(true).open(&path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub(super) fn write(p: &String, s: &String) -> Result<()> {
    // add sanitization to the path argument
    let path = Path::new(&p);
    let mut file = File::options().write(true).create(true).open(&path)?;

    file.write_all(s.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod io {
    use std::fs::File;
    use super::*;

    #[test]
    fn file_write() {
        let path = String::from("tests/files/test.txt");
        let _ = std::fs::remove_file(&path); 
        let test = String::from("this is a test");
        write(&path, &test).unwrap();

        // Read in file to chek.
        let mut file = match File::open(&path){
            Err(why) => panic!("could not write to file: {}", why),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s){
            Err(why) => panic!("could not read file: {}", why),
            Ok(_) => assert_eq!(s, test), 
        };
    }

    #[test]
    fn file_read() {
        let path = String::from("tests/files/test2.txt");
        let test = String::from("this is a test");
        let file = read(&path).unwrap();
        assert_eq!(file, test);
    }
}
