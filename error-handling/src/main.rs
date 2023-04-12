use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    open_file()
}

fn open_file() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(error) => {
                panic!(
                    "Tried to create a new file but failed with a error {:?}",
                    error
                )
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file {:?}", error)
        }
    };
    println!("{:?}", f);

    match easy_read_file() {
        Ok(fc) => println!("{:?}", fc),
        Err(error) => {
            panic!("{:?}", error)
        }
    }
}

// this function should have to have return type otherwise rust won't know how to treat "?"
fn easy_read_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
