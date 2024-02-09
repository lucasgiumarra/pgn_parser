use std::io::{self, Read};
use std::fs::File;
use std::path::PathBuf;
use pgn_reader::BufferedReader;
use read_pgn::GameCounter; 

mod read_pgn;
fn main() -> io::Result<()> {
    // Build the file path
    let mut file_path = PathBuf::from(std::env::current_dir()?);
    file_path.push("master_games.pgn");

    // Open the file
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => {
            // Print the error for debugging purposes
            println!("Error opening file: {:?}", e);
            return Err(e);
        }
    };
    
    // Read the content of the file into a String
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Create a reader from the file content
    let mut reader = BufferedReader::new_cursor(&content);

   
    // Create a visitor
    let mut visitor = GameCounter::new();

    // Create a game counter 
    let mut game_number = 1; 

    // Print game 1
    println!("Game {}: ", game_number);
    // Iterate through each game in the PGN file using into_iter
    while let Some(_) = reader.read_game(&mut visitor)? {
        // Print the results for each game
        game_number += 1; 
        println!("\nGame {}: ", game_number);    
    }
    Ok(())
}


