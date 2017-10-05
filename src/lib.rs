use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str;

pub struct Config{
    file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let file_name = args[1].clone();

        Ok(Config{
            file_name
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.file_name)?;
    let mut buf = [0; 4096];

    let mut wc = 0;
    let mut last_was_whitespace = true;
    loop {
        let bytes = file.read(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }

        for &i in &buf[..bytes] {
            if i == b' ' || i == b'\n' || i == b'\r' || i == b'\t' {
                if !last_was_whitespace {
                    wc += 1;
                }
                last_was_whitespace = true;
            }else{
                last_was_whitespace = false;
            }
        }
    }

    println!("Words in file: {}", wc);

    Ok(())
}