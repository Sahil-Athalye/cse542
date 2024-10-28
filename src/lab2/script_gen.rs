//script_gen.rs
//Sahil Athalye: a.sahil@wustl.edu
//declares PlayConfig type, additional constants, and functions to read and process the configuration file to generate the script

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type PlayConfig = Vec<(String, String)>;

const CONFIG_TITLE_INDEX:usize = 0;
const CONFIG_CHAR_INDEX:usize = 1;
const CONFIG_LINE_NUM:usize = 2;

const CHAR_NAME_INDEX:usize = 0;
const FILE_NAME_INDEX:usize = 1;

const min_NUM:usize = 2;

const MIN_LINE_LEN:usize = 1;

const DONE_READING:usize = 0;

fn add_script_line(play:&mut Play, unparsed_line:&String, part_name:&String){
    if unparsed_line.len() >= MIN_LINE_LEN {
        if let Some((token, rest)) = unparsed_line.split_once(char::is_whitespace) { //split line
            let trimmed_rest:String = rest.trim().to_string(); //trim whitespace
            if let Ok(number) = token.parse::<usize>(){ //parse first string (line #) into usize
                play.push((number,part_name.clone(),trimmed_rest));
            }
            else{ //if first string cannot be parsed into usize
                if SHOULD_COMPLAIN.load(Ordering::SeqCst){
                    eprintln!("Token {} does not represent a valid usize value.",token);
                }
            }
        }
    }
}

fn grab_trimmed_file_lines(file_name:&String,lines:&mut Vec<String>) -> Result<(), u8>{
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

fn process_config(play:&mut Play, play_config:&PlayConfig) -> Result<(), u8>{
    for config in play_config{ //for each character + character file
        let mut lines_read:Vec<String> = Vec::new();
        match config{
            (character_name,file_name) => if let Err(_e) = grab_trimmed_file_lines(&file_name,&mut lines_read){
                return Err(SCRIPT_GEN_FAILURE);
            }
            else{
                for line in &lines_read{ //Add character's lines into the script
                    add_script_line(play, &line ,&character_name);
                }
                
            },
        }
    }
    return Ok(());
}

fn add_config(config_line:&String, play_config:&mut PlayConfig){
    let whitespace_tokens:Vec<&str> = (config_line.split_whitespace()).collect(); //split config lines into their two strings

    if whitespace_tokens.len() < CONFIG_TOKEN_NUM {
        if SHOULD_COMPLAIN.load(Ordering::SeqCst){
            eprintln!("Less than two tokens in config line {}",config_line);
        }
    }

    if whitespace_tokens.len() > CONFIG_TOKEN_NUM {
        if SHOULD_COMPLAIN.load(Ordering::SeqCst){
            eprintln!("More than two tokens in config line {}",config_line);
        }
    }

    if whitespace_tokens.len() >= CONFIG_TOKEN_NUM { 
        play_config.push((whitespace_tokens[CHAR_NAME_INDEX].to_string(),whitespace_tokens[FILE_NAME_INDEX].to_string()));
    }
}

fn read_config(config_name:&String, play_title:&mut String, play_config:&mut PlayConfig) -> Result<(), u8>{
    let mut lines_read:Vec<String> = Vec::new();
    if let Err(_e) = grab_trimmed_file_lines(&config_name, &mut lines_read){
        return Err(SCRIPT_GEN_FAILURE);
    }

    if lines_read.len() < CONFIG_LINE_NUM {
        return Err(SCRIPT_GEN_FAILURE);
    }
    else{
        let mut i = CONFIG_TITLE_INDEX;
        for line in lines_read{
            if i == CONFIG_TITLE_INDEX{//if we are reading the play title
                *play_title = line.clone().to_string();
            }
            else{
                add_config(&line,play_config);
            }   
            i+=CONFIG_CHAR_INDEX;
        }
    }
    return Ok(());
}

fn script_gen( config_name:&String, play_name:&mut String, the_play:&mut Play) -> Result<(), u8>{

    let mut play_config:PlayConfig = PlayConfig::new();
    if let Err(_e) = read_config(&config_name, play_name,&mut play_config){
        return Err(SCRIPT_GEN_FAILURE);
    }

    if let Err(_e) = process_config(the_play,&play_config){
        return Err(SCRIPT_GEN_FAILURE);
    }

    return Ok(());
}



