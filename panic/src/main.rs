use std::{
    fs::File,
    io::{
        ErrorKind,
        self,
        Read,
    }
};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error);
            }
        },
    };

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open file");
}

// 传播错误
fn read_message_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(error) => Err(error),
    // }

    //——————————————————————————————————————————

    // let mut f = File::open("hello.txt")?;

    // let mut s = String::new();

    // f.read_to_string(&mut s)?;
    // Ok(s)

    // 链式方法调用
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 10 || value > 100 {
            panic!("Guess must be between 10 and 100, but got {}.", value);
        }
        Guess { value }
    }

    pub fn get(&self) -> i32 {
        self.value
    }
}