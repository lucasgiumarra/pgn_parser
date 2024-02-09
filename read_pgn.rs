
use pgn_reader::{Visitor, Skip, RawHeader, SanPlus};

pub struct GameCounter {
    moves: f32,
    results: Option<String>,
    current_event: Option<String>,
    current_player_white: Option<String>,
    current_player_black: Option<String>,
    game_moves: Vec<String>,
    move_number: u16,
}

impl GameCounter {
    pub fn new() -> GameCounter {
        GameCounter {
            moves: 0.0,
            results: None,
            current_event: None,
            current_player_white: None,
            current_player_black: None,
            game_moves: Vec::new(),
            move_number: 1,
        }
    }

}

impl Visitor for GameCounter {
    type Result = ();

    fn begin_game(&mut self) {
        // Reset the values at the start of each game
        self.moves = 0.0;
        self.results = None;
        self.current_event = None; 
        self.current_player_white = None; 
        self.current_player_black = None; 
        self.game_moves.clear();
        self.move_number = 1;
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        match key {
            b"Event" => {
                if let Ok(value_str) = std::str::from_utf8(value.0) {
                    self.current_event = Some(value_str.to_string());
                } else {
                    println!("Non-UTF-8 value for 'Event' header");
                }
            }
            b"White" => {
                if let Ok(value_str) = std::str::from_utf8(value.0) {
                    self.current_player_white = Some(value_str.to_string());
                } else {
                    println!("Non-UTF-8 value for 'White' header");
                }
            }
            b"Black" => {
                if let Ok(value_str) = std::str::from_utf8(value.0) {
                    self.current_player_black = Some(value_str.to_string());
                } else {
                    println!("Non-UTF-8 value for 'Black' header");
                }
            }
            b"Result" => {
                if let Ok(value_str) = std::str::from_utf8(value.0) {
                    self.results = Some(value_str.to_string());
                } else {
                    println!("Non-UTF-8 value for 'Black' header");
                }
            }
            _ => {} // Handle other cases if needed
        }

    }

    fn san(&mut self, san_plus: SanPlus) {
        self.moves += 0.5;
    
        let formatted_move = if self.moves % 1.0 != 0.0 { 
            format!("{}. {}", self.move_number, san_plus.san)
        } else { 
            format!(" {}", san_plus.san)
        };
    
        self.game_moves.push(formatted_move);
    
        if self.moves % 1.0 == 0.0 { 
            self.move_number += 1;
        }
    }
    

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn end_game(&mut self) -> Self::Result {
        // Return the results for each game along with the event header
        match (
            &self.results,
            &self.current_event,
            &self.current_player_white,
            &self.current_player_black,
        ) {
            (Some(result),Some(event), Some(white_pieces), Some(black_pieces)) => {
                println!("\tResult: {}", result);
                println!("\tEvent: {}", event);
                println!("\tWhite pieces: {}", white_pieces);
                println!("\tBlack pieces: {}", black_pieces);
            }
            _ => {},
        }
        let total_moves = self.moves.ceil() as u16;
        println!("\tTotal moves: {}", total_moves);
        println!("\tMoves: {}", self.game_moves.join(" "));

        // Reset the internal state for the next game
        self.moves = 0.0;
        self.results = None;
        self.current_event = None;
        self.current_player_white = None;
        self.current_player_black = None;

        //results
    }
}
