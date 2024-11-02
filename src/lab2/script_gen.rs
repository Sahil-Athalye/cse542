//script_gen.rs
//Sahil Athalye: a.sahil@wustl.edu 
//Varad Deouskar: varad@wustl.edu
//Implements the grab trimmed file lines function

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use crate::lab2::declarations::SCRIPT_GEN_FAILURE;

const DONE_READING:usize = 0;

pub fn grab_trimmed_file_lines(file_name:&String,lines:&mut Vec<String>) -> Result<(), u8>{
    match File::open(file_name) {
        Err(_e) => {
            println!("couldn't open file named {}", file_name);
            return Err(SCRIPT_GEN_FAILURE);
        }
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut lines_read = String::new();

            loop{ //read lines until done reading [read_line() returns 0]
                match reader.read_line(&mut lines_read) {
                    Err(_e) => return Err(SCRIPT_GEN_FAILURE),
                    Ok(res) => {
                        if res==DONE_READING{
                            return Ok(());
                        }
                        lines.push(lines_read.trim().to_string());
                        lines_read.clear();
                        
                    }
                };
                
            }
            

        }
    };
}