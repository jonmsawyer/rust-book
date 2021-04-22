//#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(unused_imports)]

//#[allow(unused_imports)]
use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;


#[allow(dead_code)]
fn read_username_from_file4(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

#[allow(dead_code)]
fn read_username_from_file3(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(filename)?.read_to_string(&mut s)?;

    Ok(s)
}

#[allow(dead_code)]
fn read_username_from_file2(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
//fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
fn read_username_from_file(filename: &str) -> Result<String, String> {
    // Using match expressions
    //let f = File::open(filename);
    //let mut f = match f {
    //    Ok(file) => file,
    //    Err(error) => match error.kind() {
    //        ErrorKind::NotFound => match File::create(filename) {
    //            Ok(fc) => fc,
    //            Err(e) => panic!("Problem creating the file: {:?}", e),
    //        },
    //        other_error => {
    //            panic!("Problem opening the file: {}. {:?}", filename, other_error)
    //        }
    //    },
    //};
    
    // No match expressions (experienced Rustacean might use this instead)
    let mut f = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    let s: String = match read_username_from_file("username.txt") {
        Ok(s) => s,
        Err(e) => String::from(format!("Error opening file: {}", e)),
    };
    
    println!("Username is: {}", s);
    
    let s: String = match read_username_from_file2("username.txt") {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };
    
    println!("Username is: {}", s);
    
    let s: String = match read_username_from_file3("username.txt") {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };
    
    println!("Username is: {}", s);
    
    let s: String = match read_username_from_file4("username.txt") {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };
    
    println!("Username is: {}", s);
}

//fn main() -> Result<(), Box<dyn Error>> {
//    let _f = File::open("username.txt")?;
//
//    Ok(())
//}
