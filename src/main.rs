//! # Tic Tac Toe
//! A simple command line based tic tac toe game.
//!
//! author:  MoirtzKn
//! licence: MIT

use std::io;
use std::io::Write;


#[derive(PartialEq, Eq, Copy, Clone)]
enum FieldState {
    None,
    Cross,
    Circle,
}

impl FieldState {
    fn toggle(&mut self) {
        *self = match self {
            &mut FieldState::Cross => FieldState::Circle,
            &mut FieldState::Circle => FieldState::Cross,
            _ => FieldState::None,
        };
    }

    fn to_str(&self) -> &str {
        return match self {
            &FieldState::Cross => "cross",
            &FieldState::Circle => "circle",
            &FieldState::None => "none",
        };
    }
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

type Pitch = [[FieldState; 3]; 3];

fn main(){
    println!("Tic Tac Toe");

    let mut pitch = [[FieldState::None; 3]; 3];
    let mut active_player = FieldState::Circle;
    let mut round_index = 0;
    let mut winner = FieldState::None;

    display_pitch(&pitch);

    while winner == FieldState::None {
        active_player.toggle();
        round_index += 1;

        println!("\n*****************");
        println!("round:  {}", round_index);
        println!("player: {}", active_player.to_str());
        print!("move:   ");

        let mut player_move;
        loop {
            player_move = get_move();

            if get_field_in_pitch(&pitch, &player_move) != FieldState::None {
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

    println!("Player {} won after {} rounds", winner.to_str(), round_index);
}


// paint a tic tac toe pitch with ASCII characters and show the current game state
fn display_pitch (pitch : &Pitch) {

    // the column in indices
    println!("   1   2   3 ");

    for y in 0..3 {
        if y != 0 {
            print!("  ---+---+---\n");
        }

        // the row in index
        print!("{} ", y + 1);

        for x in 0..3 {
            if x != 0 {
                print!("|");
            }

            match pitch[x][y] {
                FieldState::None => print!("   "),
                FieldState::Cross => print!(" X "),
                FieldState::Circle => print!(" O "),
            }
        }
        println!("");
    }
}

/// get the next move from stdin
fn get_move () -> Position {
    let mut first_try = true;

    loop {
        if !first_try {
            print!("invalid input, please try again: ");
        }
        first_try = false;

        let mut input = String::new();

        // flush stdout because we used 'print!', which doesn't auto flush
        io::stdout().flush()
            .ok().expect("flush() fail");
        io::stdin().read_line(&mut input)
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
        return Position { x: x-1, y: y-1 };
    }
}

fn get_field_in_pitch(pitch: &Pitch, pos: &Position) -> FieldState {
    pitch[pos.x][pos.y]
}

fn set_field_in_pitch(pitch: &mut Pitch, pos: &Position, state: &FieldState) {
    pitch[pos.x][pos.y] = *state;
}

/// seach for 3 equal fields in a row.
/// if there is a row, it returns the "row owner".
/// if there is not row it returns FieldState::None.
fn get_winner(pitch: &Pitch) -> FieldState {

    // test if 3 fields are equal
    fn row_owner(a: FieldState, b: FieldState, c: FieldState) -> FieldState {
        if a == b && b == c {
            return a;
        }
        return FieldState::None;
    }

    // variable to remember possible winners
    let mut test_owner;

    for a in 0..3 {
        // test all columns
        test_owner = row_owner(pitch[a][0], pitch[a][1], pitch[a][2]);
        if test_owner != FieldState::None {
            return test_owner;
        }

        // test all rows
        test_owner = row_owner(pitch[0][a], pitch[1][a], pitch[2][a]);
        if test_owner != FieldState::None {
            return test_owner;
        }
    }

    // test top left to bottom right
    test_owner = row_owner(pitch[0][0], pitch[1][1], pitch[2][2]);
    if test_owner != FieldState::None {
        return test_owner;
    }

    // test top right to bottom left
    test_owner = row_owner(pitch[2][0], pitch[1][1], pitch[0][2]);
    if test_owner != FieldState::None {
        return test_owner;
    }

    return FieldState::None;
}
