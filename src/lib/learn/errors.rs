use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

pub fn errors() {
    //  Panik defaults to unwind, cleaning up stack frames
    //  change to abort in profile to abort

    let filee = File::open("hello.txt");
    let filee = match filee {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    let f = File::open("hello.txt").unwrap();
    let g = File::open("hello.txt").expect("Failed to open hello.txt");

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_username_improved() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        // Chaining method calls
        File::open("hello.txt")?.read_to_string(&mut s)?;
        // ? can only be used with return type Result, Option, implementing Try
        Ok(s)
    }

    fn read_with_fn() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}
