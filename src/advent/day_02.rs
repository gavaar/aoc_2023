use std::fs;
mod game;
pub use game::{Game, Round};

fn parse_input(file: &str, bag: Round) -> Vec<Game>  {
  let read_file = fs::read_to_string(file).expect("error reading file");
  let mut games: Vec<Game> = Vec::new();

  for line in read_file.lines() {
    games.push(Game::new(line, &bag));
  }

  games
}

fn add_game_ids(file: &str) -> usize {
  let games = parse_input(file, Round { blue: 14, green: 13, red: 12 });
  let mut count = 0;
  for game in games {
    if game.is_possible {
      count += game.id;
    }
  }
  count
}

fn add_game_powers(file: &str) -> usize {
  let games = parse_input(file, Round { blue: 0, green: 0, red: 0 });
  let mut count = 0;
  for game in games {
    count += game.power;
  }
  count
}

pub fn run() {
  println!("### PART ONE ###");
  println!("-- Test --");
  println!("The value is: {}", add_game_ids("src/advent/day_02/test"));
  println!("The power sum is: {}", add_game_powers("src/advent/day_02/test"));
  
  println!("-- Solution part 1 --");
  println!("The value of game 1 is: {}", add_game_ids("src/advent/day_02/input"));
  println!("The power sum is: {}", add_game_powers("src/advent/day_02/input"));
}