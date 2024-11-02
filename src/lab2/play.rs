//play.rs
//Sahil Athalye: a.sahil@wustl.edu 
//Varad Deouskar: varad@wustl.edu
//declares and implements the Play struct which handles the partial config files

use std::sync::atomic::Ordering;
use crate::lab2::script_gen::grab_trimmed_file_lines;
use crate::lab2::declarations::SCRIPT_GEN_FAILURE;
use crate::lab2::declarations::SHOULD_COMPLAIN;

use super::scene_fragment::SceneFragment;

type ScriptConfig = Vec<(bool, String)>;
type Fragments = Vec<SceneFragment>;

const ONE_TOKEN:usize = 1;


#[derive(Debug)]
pub struct Play{
    fragments:Fragments,
}

impl Play{
    pub fn new() -> Self {
        let fragments:Fragments = Vec::new();
        return Self { fragments};
    }

    pub fn process_config(&mut self, play_config: &ScriptConfig) -> Result<(), u8> {
        let mut current_title = String::new();
        
        for (is_title, text) in play_config {
            if *is_title {
                // Update the current title if this is a scene title
                current_title = text.clone();
            } else {
                // Create new fragment with current title
                let mut fragment = SceneFragment::new(&current_title);
                
                // Pass the config file text to prepare method
                match fragment.prepare(text,&mut current_title) {
                    Ok(_) => {
                        self.fragments.push(fragment);
                        current_title = String::new(); // Reset title after fragment creation
                    },
                    Err(_) => return Err(1) // Return error if script generation failed
                }
            }
        }
        Ok(())
    }

    fn add_config(config_line: &String, play_config: &mut ScriptConfig) {
        // Skip blank lines
        if config_line.trim().is_empty() {
            return;
        }
    
        let tokens: Vec<&str> = config_line.split_whitespace().collect();
        
        // Skip empty token lists (shouldn't happen with trim but being defensive)
        if tokens.is_empty() {
            return;
        }
    
        // Handle [scene] directives
        if tokens[0] == "[scene]" {
            if tokens.len() == ONE_TOKEN {
                // [scene] with no title
                if SHOULD_COMPLAIN.load(Ordering::SeqCst) {
                    eprintln!("Missing scene title after [scene]");
                }
                return;
            }
            // Concatenate remaining tokens as scene title
            let scene_title = tokens[1..].join(" ");
            play_config.push((true, scene_title));
            return;
        }
    
        // Handle configuration file lines
        let config_file = tokens[0].to_string();
        if tokens.len() > ONE_TOKEN && SHOULD_COMPLAIN.load(Ordering::SeqCst) {
            eprintln!("Additional tokens after configuration file name: {}", config_line);
        }
        play_config.push((false, config_file));
    }

    fn read_config(script_name: &String, play_config: &mut ScriptConfig) -> Result<(), u8> {
        let mut lines_read: Vec<String> = Vec::new();
        
        // Attempt to read lines from the script file
        if let Err(_e) = grab_trimmed_file_lines(script_name, &mut lines_read) {
            eprintln!("Failed to open or read from script file: {}", script_name);
            return Err(SCRIPT_GEN_FAILURE);
        }
    
        // Check if any lines were read
        if lines_read.is_empty() {
            eprintln!("No lines read from script file: {}", script_name);
            return Err(SCRIPT_GEN_FAILURE);
        }
    
        // Process each line using add_config
        for line in lines_read {
            Self::add_config(&line, play_config);
        }
    
        Ok(())
    }

    pub fn prepare(&mut self, script_name: &String) -> Result<(), u8> {
        // Create a new empty ScriptConfig
        let mut script_config: ScriptConfig = ScriptConfig::new();
    
        // Read and process the configuration using the new read_config signature
        if let Err(_e) = Self::read_config(script_name, &mut script_config) {
            return Err(SCRIPT_GEN_FAILURE);
        }
    
        // Process the configuration to set up the play
        if let Err(_e) = self.process_config(&script_config) {
            return Err(SCRIPT_GEN_FAILURE);
        }
    
        // Validate that we have fragments and the first fragment has a title
        if self.fragments.is_empty() {
            eprintln!("No scene fragments were created from script file: {}", script_name);
            return Err(SCRIPT_GEN_FAILURE);
        }
    
        // Check if the first fragment has a title
        if !self.fragments.first().map_or(false, |f| !f.scene_title.is_empty()) {
            eprintln!("First scene fragment is missing a title in script file: {}", script_name);
            return Err(SCRIPT_GEN_FAILURE);
        }
    
        Ok(())
    }

    pub fn recite(&mut self) {
        // Do nothing if there are no fragments
        if self.fragments.is_empty() {
            return;
        }
    
        let last_index = self.fragments.len() - 1;
    
        for i in 0..=last_index {
            if i == 0 {
                // First fragment: use enter_all
                self.fragments[0].enter_all();
            } else {
                // Get indices for splitting the vector
                let (prev_idx, curr_idx) = (i - 1, i);
                
                // Split the vector at current index to get both mutable and immutable references
                let (left, right) = self.fragments.split_at_mut(curr_idx);
                let prev_fragment = &left[prev_idx];
                let current_fragment = &mut right[0];
                
                // Other fragments: use enter with previous fragment reference
                current_fragment.enter(prev_fragment);
            }
    
            // Recite the current fragment
            self.fragments[i].recite();
    
            if i == last_index {
                // Last fragment: use exit_all
                self.fragments[i].exit_all();
            } else {
                // Get indices for splitting the vector
                let (curr_idx, next_idx) = (i, i + 1);
                
                // Split the vector at next index to get both mutable and immutable references
                let (left, right) = self.fragments.split_at_mut(next_idx);
                let current_fragment = &mut left[curr_idx];
                let next_fragment = &right[0];
                
                // Other fragments: use exit with next fragment reference
                current_fragment.exit(next_fragment);
            }
        }
    }


}