//main.rs
//Sahil Athalye: a.sahil@wustl.edu
//Main Function: parses command line arguments, calls for script generation, and then recites script
pub mod lab2;

use std::env;
use std::sync::atomic::Ordering;

const MIN_ARGS:usize = 2;
const MAX_ARGS:usize = 3;

const PROGRAM_POSITION:usize = 0;
const CONFIG_POSITION:usize = 1;
const OPTIONALARG_POSITION:usize = 2;

const FIX_COMMAND_LINE:u8 = 1;

fn main() ->  Result<(), u8> {
    let mut config_filename:String = String::new();

    if let Err(_e) = parse_args(&mut config_filename){
        return Err(FIX_COMMAND_LINE);
    }

    let mut play_name:String = String::new();
    let mut play:Play = Vec::new();

    if let Err(_e) = script_gen(&config_filename, &mut play_name, &mut play){
        return Err(SCRIPT_GEN_FAILURE);
    }
    
    play.sort(); //order lines

    recite(&play_name,&play);

    return Ok(());
}

fn usage(program_name: &String){
    println!("usage: {} <config_file_name> [whinge]",program_name);
}

fn parse_args(config_filename:&mut String) -> Result<(), u8>{
    let mut cli_args: Vec<String> = Vec::new();
    for arg in env::args() { 
        cli_args.push(arg);
    }

    
    if cli_args.len() > MAX_ARGS { //if too many args
        usage(&cli_args[PROGRAM_POSITION]);
        return Err(FIX_COMMAND_LINE);
    }
    else if cli_args.len() < MIN_ARGS { //if too few args
        usage(&cli_args[PROGRAM_POSITION]);
        return Err(FIX_COMMAND_LINE);
    }
    else if cli_args.len() == MAX_ARGS && cli_args[OPTIONALARG_POSITION] != "whinge"{ //if optional arg is not "whinge"
        usage(&cli_args[PROGRAM_POSITION]);
        return Err(FIX_COMMAND_LINE);
    }
    else if cli_args.len() == MAX_ARGS && cli_args[OPTIONALARG_POSITION] == "whinge"{ //if optional arg is "whinge"
        SHOULD_COMPLAIN.store(true,Ordering::SeqCst);
    }

    *config_filename = cli_args[CONFIG_POSITION].clone();
    Ok(())
}

fn recite(play_title:&String, the_play:&Play){
    println!("{}",play_title);

    let mut character_name:String = " ".to_string();

    for performance in the_play {

     match performance {
       (.., character_name_temp, line) => {
 
        if character_name != *character_name_temp {   
            println!("");

              // just print out the new character's name
              println!("{}.", character_name_temp);
        }
        
        println!("{}",line);
        character_name = character_name_temp.clone();

       }
     }

    }
}
