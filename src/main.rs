//! # Tic Tac Toe
//! A simple command line based tic tac toe game.
//!
//! author:  MoirtzKn
//! licence: MIT

use std::io;
use std::io::Write;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Player {
    Empty,
    Cross,
    Circle,
}

impl Player {
    fn toggle(&mut self) {
        *self = self.get_opponent();
    }

    fn get_opponent(&self) -> Player {
        match *self {
            Player::Cross => Player::Circle,
            Player::Circle => Player::Cross,
            Player::Empty => Player::Empty,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::Cross => write!(f, "cross"),
            Player::Circle => write!(f, "circle"),
            Player::Empty => write!(f, "no one"),
        }
    }
}

/// holds information about the current playing players
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Players {
    current: Player,
    cross_is_ai: bool,
    circle_is_ai: bool,
}

impl Players {
    fn current_is_ai(&self) -> bool {
        match self.current {
            Player::Circle => self.circle_is_ai,
            Player::Cross => self.cross_is_ai,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Move {
    x: usize,
    y: usize,
}

impl Move {
    fn new(x: usize, y: usize) -> Move {
        Move { x: x, y: y }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y:{}", self.x+1, self.y+1)
    }
}

/// one line on the games field aka pitch
type Line = [Player; 3];
/// the hole game field
/// we call this pitch to avoid confusion with the single
/// square parts of the pitch which we call field
type Pitch = [Line; 3];

fn main() {
    let mut pitch = [[Player::Empty; 3]; 3];
    let mut players = Players {
        current: Player::Circle,
        cross_is_ai: false,
        circle_is_ai: false,
    };
    let mut round_index = 0;
    let mut winner = Player::Empty;

    println!("Tic Tac Toe");

    prompt_ai_options(&mut players);
    display_pitch(&pitch);

    while winner == Player::Empty && !is_draw(&pitch) {
        players.current.toggle();
        round_index += 1;

        println!("round:  {}", round_index);
        print!("player: {} ", players.current);
        if players.current_is_ai() {
            print!(" (computer player)");
        }
        println!("");
        print!("move:   ");

        let player_move;
        if players.current_is_ai() {
            if round_index == 1 {
                // because the first move is the hardest to calculate, this is just hard coded
                player_move = Move::new(0, 0);
            } else {
                player_move = get_ai_move(&pitch, players.current);
            }
            // display the AI move like the user move would
            println!("{} {}", player_move.x+1, player_move.y+1);
        } else {
            player_move = get_user_move(&pitch);
        }
        // save the players move
        set_field_in_pitch(&mut pitch, &player_move, players.current);
        // show the current game state
        display_pitch(&pitch);
        // test if there are 3 equal fields in a row
        winner = get_winner(&pitch);
    }

    if winner != Player::Empty {
        if players.current_is_ai() {
            println!("The Computer ({}) won after {} rounds", winner, round_index);
        } else {
            println!("{} won after {} rounds", winner, round_index);
        }
    } else {
        println!("Draw after {} rounds", round_index);
    }
}

/// asks the user if he or she wants to play against an AI
fn prompt_ai_options(player: &mut Players) {
    print!("Should cross be played by a computer ");
    player.cross_is_ai = prompt_confirm();
    print!("Should circle be played by a computer ");
    player.circle_is_ai = prompt_confirm();
}

/// writes "(y/n): " to stdout and waits for user input.
/// if the user enters "y" or "yes", the function returns true.
/// if the user enters "n" or "no", the function returns false.
/// if the user enters something else, the function informs
/// the user over the invaid input and waits for new input.
fn prompt_confirm() -> bool {
    loop {
        print!("(y/n): ");

        // flush stdout because we used 'print!', which doesn't auto flush
        io::stdout()
            .flush()
            .ok()
            .expect("flush() fail");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match input.trim() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {}
        };

        println!("invalid input, enter \"y\" or \"n\"");
        print!("please try again: ");
    }
}

/// paint a tic tac toe pitch with ASCII characters and show the current game state
fn display_pitch(pitch: &Pitch) {
    println!("\n*****************");
    // the column indices
    println!("   1   2   3 ");

    for y in 0..3 {
        if y != 0 {
            print!("  ---+---+---\n");
        }

        // the row index
        print!("{} ", y + 1);

        for x in 0..3 {
            if x != 0 {
                print!("|");
            }

            match pitch[x][y] {
                Player::Empty => print!("   "),
                Player::Cross => print!(" X "),
                Player::Circle => print!(" O "),
            }
        }
        println!("");
    }
    println!("\n*****************");
}

/// calculate the best possible move
fn get_ai_move(pitch: &Pitch, player_on_turn: Player) -> Move {
    // will be used to try out possible moves
    let mut tmp_pitch = pitch.clone();

    fn applay_if_higher (n: &mut i32, m: i32) {
        if m > *n {
            *n = m;
        }
    };

    fn applay_if_lower (n: &mut i32, m: i32) {
        if m < *n {
            *n = m;
        }
    };

    /// calculate a score for the current game state by recursively
    /// tying out every possible move.
    ///
    /// # params
    /// - pitch: the pitch to evaluate
    /// - test_player: the player doing the current test move (will change during recursion)
    /// - player_on_turn: the ai player (will stay the same during recursion)
    fn minmax(mut pitch: &mut Pitch, test_player: Player, player_on_turn: Player, depth: i32) -> i32 {
        let winner = get_winner(pitch);
        if winner != Player::Empty || is_draw(pitch) {
            if winner == player_on_turn {
                return 10 - depth;
            } else if winner != Player::Empty {
                return depth - 10;
            } else {
                return 0;
            }
        }

        let use_max = player_on_turn == test_player;
        let mut best_score = if use_max { -99 } else { 99 };
        for y in 0..3 {
            for x in 0..3 {
                if pitch[x][y] == Player::Empty {
                    let score = get_move_score(x, y, &mut pitch, test_player, player_on_turn, depth);
                    if use_max {
                        applay_if_higher(&mut best_score, score);
                    } else {
                        applay_if_lower(&mut best_score, score)
                    };
                }
            }
        }
        return best_score;
    }

    fn get_move_score(x: usize, y: usize, mut pitch: &mut Pitch, test_player: Player, player_on_turn: Player, depth: i32) -> i32 {
        pitch[x][y] = test_player;
        let score = minmax(&mut pitch, test_player.get_opponent(), player_on_turn, depth + 1);
        pitch[x][y] = Player::Empty;
        return score;
    }

    let mut best_move = Move::new(0, 0);
    let mut best_score = 0;
    let mut fist_possible_move = true;
    for y in 0..3 {
        for x in 0..3 {
            if pitch[x][y] == Player::Empty {
                let score = get_move_score(x, y, &mut tmp_pitch, player_on_turn, player_on_turn, 0);
                if score > best_score || fist_possible_move {
                    best_score = score;
                    best_move = Move::new(x, y);
                }
                fist_possible_move = false;
            }
        }
    }
    return best_move;
}

/// get the next move from stdin
fn get_user_move(pitch: &Pitch) -> Move {
    let mut first_try = true;

    loop {
        if !first_try {
            print!("please try again: ");
        }
        first_try = false;

        let mut input = String::new();

        // flush stdout because we used 'print!', which doesn't auto flush
        io::stdout()
            .flush()
            .ok()
            .expect("flush() fail");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        input = String::from(input.trim());

        if input.is_empty() {
            println!("no input");
            continue;
        }

        let (x_str, y_str) = input.split_at(1);

        let x: usize = match x_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Make a move by entering two numbers.");
                println!("The first number is the horizontal position the second the vertical.");
                continue;
            }
        };

        let y: usize = match y_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Make a move by entering two numbers.");
                println!("The first number is the horizontal position the second the vertical.");
                continue;
            }
        };

        if x > 3 || y > 3 {
            println!("Position has to be between 1 and 3");
            continue;
        }

        // minus one to translate from human indices to computer indices
        let player_move = Move {
            x: x - 1,
            y: y - 1,
        };

        let field_owner = get_field_in_pitch(&pitch, &player_move);

        if field_owner == Player::Empty {
            // field is empty, input accepted
            return player_move;
        }

        // the field is already set, try again
        println!("field already taken by {}", field_owner)
    }
}

fn get_field_in_pitch(pitch: &Pitch, pos: &Move) -> Player {
    pitch[pos.x][pos.y]
}

fn set_field_in_pitch(pitch: &mut Pitch, pos: &Move, state: Player) {
    pitch[pos.x][pos.y] = state;
}

/// seach for 3 equal fields in a row.
/// if there is a row, the function returns the "row owner".
/// if there is not row, it returns Player::Empty.
fn get_winner(pitch: &Pitch) -> Player {
    // test if 3 fields are equal
    fn line_owner(line: Line) -> Option<Player> {
        if line[0] != Player::Empty && line[0] == line[1] && line[1] == line[2] {
            return Some(line[0]);
        }
        return None;
    }

    for n in 0..3 {
        // test all columns
        if let Some(owner) = line_owner(pitch[n]) {
            return owner;
        }

        // test all rows
        if let Some(owner) = line_owner([pitch[0][n], pitch[1][n], pitch[2][n]]) {
            return owner;
        }
    }

    // test top left to bottom right
    if let Some(owner) = line_owner([pitch[0][0], pitch[1][1], pitch[2][2]]) {
        return owner;
    }

    // test top right to bottom left
    if let Some(owner) = line_owner([pitch[2][0], pitch[1][1], pitch[0][2]]) {
        return owner;
    }

    return Player::Empty;
}

/// check if no one is able to win in this game aka it's a draw
fn is_draw(pitch: &Pitch) -> bool {

    /// check if it's still possible to win using this line
    fn is_possible_row(line: Line) -> bool {
        let mut first_player = Player::Empty;
        for field in line.into_iter() {
            if *field != Player::Empty {
                if first_player == Player::Empty {
                    first_player = *field;
                } else if first_player != *field {
                    return false;
                }
            }
        }
        return true;
    }

    // test all columns and all rows
    for n in 0..3 {
        // test column
        if is_possible_row(pitch[n]) {
            return false;
        }

        // test row
        if is_possible_row([pitch[0][n], pitch[1][n], pitch[2][n]]) {
            return false;
        }
    }

    // test top left to bottom right
    if is_possible_row([pitch[0][0], pitch[1][1], pitch[2][2]]) {
        return false;
    }

    // test top right to bottom left
    if is_possible_row([pitch[2][0], pitch[1][1], pitch[0][2]]) {
        return false;
    }

    return true;
}
