//play.rs
//Sahil Athalye: a.sahil@wustl.edu
//DESCRIPTION

use std::sync::atomic::Ordering;
use crate::lab2::script_gen::grab_trimmed_file_lines;
use crate::lab2::declarations::SCRIPT_GEN_FAILURE;
use crate::lab2::declarations::SHOULD_COMPLAIN;
use crate::lab2::player::Player;

type PlayConfig = Vec<(String, String)>;

const CONFIG_TITLE_INDEX:usize = 0;
const CONFIG_CHAR_INDEX:usize = 1;
const CONFIG_LINE_NUM:usize = 2;

const CHAR_NAME_INDEX:usize = 0;
const FILE_NAME_INDEX:usize = 1;
const CONFIG_TOKEN_NUM:usize = 2;

#[derive(Debug)]
pub struct Play{
    scene_title:String,
    characters:Vec<Player>
}

impl Play{
    pub fn new() -> Self {
        let title:String = "".to_string();
        let players:Vec<Player> = Vec::new();
        Self { scene_title:title, characters:players}
    }

    pub fn process_config(&mut self, play_config: &PlayConfig) -> Result<(), u8> {
        for (character_name, file_name) in play_config {
            // Create a new Player instance with the character name.
            let mut player = Player::new(character_name.clone());

            // Attempt to prepare the player with the part file name.
            if let Err(error) = player.prepare(file_name) {
                return Err(error); // Return error if prepare fails.
            }

            // Add the prepared player to the Play's characters.
            self.characters.push(player);
        }
        
        Ok(())
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
        if let Err(_e) =  grab_trimmed_file_lines(&config_name, &mut lines_read){
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
                    Self::add_config(&line,play_config);
                }   
                i+=CONFIG_CHAR_INDEX;
            }
        }
        return Ok(());
    }

    pub fn prepare(&mut self, config_name: &String, play_name: &mut String) -> Result<(), u8> {
        // Initialize an empty PlayConfig.
        let mut play_config: PlayConfig = PlayConfig::new();

        // Read the configuration file and populate play_name and play_config.
        if let Err(_e) = Self::read_config(config_name, play_name, &mut play_config) {
            return Err(SCRIPT_GEN_FAILURE);
        }

        // Process the configuration to set up the play.
        if let Err(_e) = self.process_config(&play_config) {
            return Err(SCRIPT_GEN_FAILURE);
        }

        Ok(())
    }

    pub fn recite(&mut self) {
        // Print the title of the play.
        println!("{}", self.scene_title);

        let mut current_character = String::new();
        let mut expected_line_number = 0;

        loop {
            // Find the next player with the lowest line number to speak.
            let mut next_player: Option<&mut Player> = None;
            let mut next_line_number = usize::MAX;

            for player in &mut self.characters {
                if let Some(line_number) = player.next_line() {
                    // Check for the lowest line number in the players.
                    if line_number < next_line_number {
                        next_player = Some(player);
                        next_line_number = line_number;
                    }
                }
            }

            // If no player has a next line, we are done reciting.
            if next_player.is_none() {
                break;
            }

            // Handle missing lines if `SHOULD_COMPLAIN` mode is on.
            if next_line_number > expected_line_number {
                if SHOULD_COMPLAIN.load(Ordering::SeqCst) {
                    for missing_line in expected_line_number..next_line_number {
                        eprintln!("Warning: Missing line number {}", missing_line);
                    }
                }
            } else if next_line_number < expected_line_number && SHOULD_COMPLAIN.load(Ordering::SeqCst) {
                // Complain about duplicate line numbers if they are lower than expected.
                eprintln!("Warning: Duplicate line number {}", next_line_number);
            }

            // Have the selected player speak their next line, updating the current character's name.
            if let Some(player) = next_player {
                player.speak(&mut current_character);
            }

            // Update the expected line number to the next one.
            expected_line_number = next_line_number + 1;
        }
    }


}