//script_gen.rs
//Sahil Athalye: a.sahil@wustl.edu
//declares PlayConfig type, additional constants, and functions to read and process the configuration file to generate the script

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





// pub fn script_gen( config_name:&String, play_name:&mut String, the_play:&mut Play) -> Result<(), u8>{

//     let mut play_config:PlayConfig = PlayConfig::new();
//     if let Err(_e) = read_config(&config_name, play_name,&mut play_config){
//         return Err(SCRIPT_GEN_FAILURE);
//     }

//     if let Err(_e) = process_config(the_play,&play_config){
//         return Err(SCRIPT_GEN_FAILURE);
//     }

//     return Ok(());
// }



