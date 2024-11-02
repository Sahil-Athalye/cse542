1. CSE 542 Fall 2024 Lab 2

2. Sahil Athalye: a.sahil@wustl.edu
Varad Deouskar: varad@wustl.edu

3. MODULES: 
    - Nearly all constant variable initializations in declarations.rs were moved to main.rs as most of them were exclusively used in main.rs. The Play type declaration, SCRIPT_GEN_FAILURE error code, and SHOULD_COMPLAIN atomic boolean were all declared public as they were used in multiple files.

    - To ensure compilation, a variety of use statements with the crate paths to the desired variables were included at the top of script_gen.rs and main.rs. The script gen function was also required to be included in main.rs from script_gen.rs. 

    - There were three points where the script could whinge. In add_config() if there were and incorrect number of config tokens in a line and in add_script_line() if the token could not be parsed into a number. Instead of printing to the standard output stream, whinges will now print to the standard error stream. 

    - A key design decision was to not combine the helper functions and script_gen function into main.rs even though script_gen is only used called in main.rs. This decision is being made to compartmentalize the code better and allow an easier understanding of the two distinct processing steps that take place to make this program possible. 

4. Structs: 

    - In our refactoring, we structured the code around two main structs, Play and Player, to organize the play's data and behavior more effectively. The Player struct was designed to hold each character's name, lines (stored in PlayLines), and the index of the current line. Within the Player struct, we implemented several methods. The add_script_line method, a private function, adds a parsed line to the player’s PlayLines. The public prepare method reads and processes each character’s lines by calling add_script_line and then sorting the lines to account for any out-of-order entries. The speak method prints the player’s next line, checks for character name changes, and updates the current line index, while the next_line method returns the number of the upcoming line or None if there are no more lines.

    -  The Play struct itself holds the scene title and a vector of Player instances and includes methods that operate on these. We designed process_config to iterate through the play configuration, creating and preparing a new Player instance for each character, and pushing each player into the characters vector. This method returns an error if any prepare call fails, ensuring robust error handling. The recite method in Play delivers each line in order by checking the next line number across all players. In addition, it manages missing and duplicate lines as specified, logging any issues to the error stream if whinge mode is enabled.

    - During the refactoring process, we encountered several design challenges. A primary challenge was handling mutable borrowing correctly when iterating through players in recite. To resolve this, we ensured that next_player was a mutable reference and used &mut self.characters in the loop to enable mutable access. Another challenge was handling missing and duplicate lines, which we addressed by using Option types and conditional checks within recite to control the flow of line delivery. Any missing line numbers are incremented accordingly, and any discrepancies are logged when whinge mode is active.
    
    - This struct-based approach not only helped us manage ownership and borrowing effectively, as required by Rust, but also created a more modular and cohesive design, where each struct is responsible for its own data and associated behavior. This led to a clear, maintainable structure that supports ordered recitation of lines in the play.