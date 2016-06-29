//! # Tic Tac Toe
//! A simple command line based tic tac toe game.
//!
//! author:  MoirtzKn
//! licence: MIT

use std::io;
use std::io::Write;
use std::fmt;
use FieldState::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum FieldState {
    Empty,
    Cross,
    Circle,
}

impl FieldState {
    fn toggle(&mut self) {
        *self = match *self {
            Cross => Circle,
            Circle => Cross,
            _ => Empty,
        };
    }
}

impl fmt::Display for FieldState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cross => write!(f, "cross"),
            Circle => write!(f, "circle"),
            Empty => write!(f, "empty"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

type Line = [FieldState; 3];
type Pitch = [Line; 3];

fn main() {
    println!("Tic Tac Toe");

    let mut pitch = [[Empty; 3]; 3];
    let mut active_player = Circle;
    let mut round_index = 0;
    let mut winner = Empty;

    display_pitch(&pitch);

    while winner == Empty && !is_draw(&pitch) {
        active_player.toggle();
        round_index += 1;

        println!("\n*****************");
        println!("round:  {}", round_index);
        println!("player: {}", active_player);
        print!("move:   ");

        let mut player_move;
        loop {
            player_move = get_move();

            if get_field_in_pitch(&pitch, &player_move) != Empty {
                // the field is already set, try again
                print!("field already taken, please try again: ");
                continue;
            } else {
                // field is empty, input accepted
                break;
            }
        }

        // save the players move
        set_field_in_pitch(&mut pitch, &player_move, &active_player);
        // show the current game state
        display_pitch(&pitch);

        // test if there are 3 equal fields in a row
        winner = get_winner(&pitch);
    }

    if winner != Empty {
        println!("{} won after {} rounds", winner, round_index);
    } else {
        println!("Draw after {} rounds", round_index);
    }
}


// paint a tic tac toe pitch with ASCII characters and show the current game state
fn display_pitch(pitch: &Pitch) {

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
                Empty => print!("   "),
                Cross => print!(" X "),
                Circle => print!(" O "),
            }
        }
        println!("");
    }
}

/// get the next move from stdin
fn get_move() -> Position {
    let mut first_try = true;

    loop {
        if !first_try {
            print!("invalid input, please try again: ");
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

        if input.trim().is_empty() {
            continue;
        }

        let (x_str, y_str) = input.split_at(1);

        let x: usize = match x_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let y: usize = match y_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if x > 3 || y > 3 {
            continue;
        }

        // minus one to translate from human indices to computer indices
        return Position {
            x: x - 1,
            y: y - 1,
        };
    }
}

fn get_field_in_pitch(pitch: &Pitch, pos: &Position) -> FieldState {
    pitch[pos.x][pos.y]
}

fn set_field_in_pitch(pitch: &mut Pitch, pos: &Position, state: &FieldState) {
    pitch[pos.x][pos.y] = *state;
}

/// seach for 3 equal fields in a row.
/// if there is a row, the function returns the "row owner".
/// if there is not row, it returns FieldState::Empty.
fn get_winner(pitch: &Pitch) -> FieldState {
    // test if 3 fields are equal
    fn row_owner(a: FieldState, b: FieldState, c: FieldState) -> Option<FieldState> {
        if a != Empty && a == b && b == c {
            return Some(a);
        }
        return None;
    }

    for a in 0..3 {
        // test all columns
        if let Some(owner) = row_owner(pitch[a][0], pitch[a][1], pitch[a][2]) {
            return owner;
        }

        // test all rows
        if let Some(owner) = row_owner(pitch[0][a], pitch[1][a], pitch[2][a]) {
            return owner;
        }
    }

    // test top left to bottom right
    if let Some(owner) = row_owner(pitch[0][0], pitch[1][1], pitch[2][2]) {
        return owner;
    }

    // test top right to bottom left
    if let Some(owner) = row_owner(pitch[2][0], pitch[1][1], pitch[0][2]) {
        return owner;
    }

    return Empty;
}

/// Check if no one is able to win in this game aka it's a draw
fn is_draw(pitch: &Pitch) -> bool {

    /// Check if it's still possible to win using this line
    fn is_possible_row(line: Line) -> bool {
        let mut first_player = Empty;
        for field in line.into_iter() {
            if *field != Empty {
                if first_player == Empty {
                    first_player = *field;
                } else if first_player != *field {
                    return false;
                }
            }
        }
        return true;
    }


    // test all columns and all rows
    for a in 0..3 {
        // test column
        if is_possible_row(pitch[a]) {
            return false;
        }

        // test row
        if is_possible_row([pitch[0][a], pitch[1][a], pitch[2][a]]) {
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
