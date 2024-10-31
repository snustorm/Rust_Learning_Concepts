use std::fs::File;
use std::io::{self, ErrorKind, Read};


pub fn create_file_and_error_handle () {

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    //上面等于下面这个👇

    let f1 = File::open("hello.txt").unwrap();


    //----------------------------------分割线-------------------------------

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    let f = File::open("hello.txt").unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error); 
        }
    });
}


fn read_username_from_file() -> Result<String, io::Error> {

    let f = File::open("hello.tsx");

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

//上面等于下面
fn read_username_from_file1() -> Result<String, io::Error> {

    let mut s = String::new();
    File::open("hello.tsx")?.read_to_string(&mut s)?;
    Ok(s)
}