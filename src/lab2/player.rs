//player.rs
//Sahil Athalye: a.sahil@wustl.edu
//DESCRIPTION


use crate::lab2::script_gen::grab_trimmed_file_lines;
use std::sync::atomic::Ordering;
use crate::lab2::declarations::SCRIPT_GEN_FAILURE;
use crate::lab2::declarations::SHOULD_COMPLAIN;

const MIN_LINE_LEN:usize = 1;

// pub type Play = Vec<(usize, String, String)>;

//usize for line number and String for line itself
pub type PlayLines = Vec<(usize, String)>;

#[derive(Debug)]
pub struct Player{
    name:String,
    lines:PlayLines,
    cur_idx:usize,
}

impl Player{
    pub fn new(character_name:String) -> Self {
        let play_lines:PlayLines = Vec::new();
        Self { name:character_name, lines:play_lines, cur_idx:0}
    }

    fn add_script_line(&mut self, unparsed_line: &String) {
        if unparsed_line.len() >= MIN_LINE_LEN {
            if let Some((token, rest)) = unparsed_line.split_once(char::is_whitespace) { //split line
                let trimmed_rest: String = rest.trim().to_string(); //trim whitespace
                if let Ok(number) = token.parse::<usize>() { //parse first string (line #) into usize
                    self.lines.push((number, trimmed_rest));
                } else if SHOULD_COMPLAIN.load(Ordering::SeqCst) { //if first string cannot be parsed into usize
                    eprintln!("Token {} does not represent a valid usize value.", token);
                }
            }
        }
    }

    pub fn prepare(&mut self, file_name: &String) -> Result<(), u8> {
        let mut lines_read: Vec<String> = Vec::new();

        // Attempt to read and trim lines from the specified file.
        if let Err(_e) = grab_trimmed_file_lines(file_name, &mut lines_read) {
            return Err(SCRIPT_GEN_FAILURE); // Return failure if file read fails.
        }

        // Process each line read from the file using add_script_line.
        for line in &lines_read {
            self.add_script_line(line);
        }

        // Sort the lines container by line number.
        self.lines.sort_by_key(|(number, _)| *number);

        Ok(())
    }


    pub fn speak(&mut self, last_speaker: &mut String) {
        // Check if the current index is within the bounds of the lines.
        if self.cur_idx >= self.lines.len() {
            return; // Simply return if out of bounds.
        }

        // Check if the last speaker was different.
        if *last_speaker != self.name {
            // Update the last speaker to the current player's name.
            *last_speaker = self.name.clone();
            // Print a blank line and then the player's name.
            println!();
            println!("{}", self.name);
        }

        // Print the line at the current index.
        println!("{}", self.lines[self.cur_idx].1);
        
        // Move to the next line.
        self.cur_idx += 1;
    }

    pub fn next_line(&self) -> Option<usize> {
        // Check if the current index is within the bounds of the lines.
        if self.cur_idx < self.lines.len() {
            // Return the line number at the current index.
            Some(self.lines[self.cur_idx].0)
        } else {
            // Return None if out of bounds.
            None
        }
    }

}