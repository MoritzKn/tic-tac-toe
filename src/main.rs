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
#[derive(Debug, Copy, Clone)]
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

    fn toggle(&mut self) {
        self.current = self.current.get_opponent();
    }

    fn get_next_move(&mut self, board: &Board) -> Move {
        if !self.current_is_ai() {
            self.get_user_move(&board)
        } else {
            self.get_ai_move(&board)
        }
    }

    /// calculate the best possible move for the current player
    fn get_ai_move(&self, board: &Board) -> Move {

        /// apply the value of m to n if m is larger than n
        fn applay_if_higher (n: &mut i32, m: i32) {
            if m > *n {
                *n = m;
            }
        }

        /// apply the value of m to n if m is smaller than n
        fn applay_if_lower (n: &mut i32, m: i32) {
            if m < *n {
                *n = m;
            }
        }

        /// calculate a score for the current game state by recursively
        /// tying out every possible move.
        ///
        /// # params
        /// - board:          the board to evaluate
        /// - test_player:    the player doing the current test move (will change during recursion)
        /// - player_on_turn: the ai player (will stay the same during recursion)
        /// - depth:          the current depth of the regression
        ///
        /// # return
        /// the function will return a score from -10 to 10
        fn minmax(mut board: &mut Board, test_player: Player, player_on_turn: Player, depth: i32) -> i32 {
            let winner = board.get_winner();
            if winner != Player::Empty || board.is_draw() {
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
                    let player_move = Move::new(x, y);
                    if board.get_field(&player_move) == Player::Empty {
                        let score = get_move_score(&player_move, &mut board, test_player, player_on_turn, depth);
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

        // helper function for the minmax function
        fn get_move_score(player_move: &Move, mut board: &mut Board, test_player: Player, player_on_turn: Player, depth: i32) -> i32 {
            board.set_field(&player_move, test_player);
            let score = minmax(&mut board, test_player.get_opponent(), player_on_turn, depth + 1);
            board.set_field(&player_move, Player::Empty);
            return score;
        }

        // will be used to try out possible moves
        let mut tmp_board = board.clone();

        let mut best_move = Move::new(0, 0);
        let mut best_score = 0;
        let mut fist_possible_move = true;
        for y in 0..3 {
            for x in 0..3 {
                let player_move = Move::new(x, y);
                if board.get_field(&player_move) == Player::Empty {
                    let score = get_move_score(&player_move, &mut tmp_board, self.current, self.current, 0);
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
    fn get_user_move(&self, board: &Board) -> Move {
        let mut first_try = true;

        loop {
            if !first_try {
                print!("please try again: ");
            } else {
                first_try = false;
            }

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

            let x: usize = match x_str.trim() {
                "a"|"A" => 1,
                "b"|"B" => 2,
                "c"|"C" => 3,
                _ => {
                    println!("You have to enter A, B or C and a number");
                    continue;
                }
            };

            let y: usize = match y_str.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("The horizontal position has to be between 1 and 3");
                    continue;
                }
            };

            if y > 3 {
                println!("The horizontal position has to be between 1 and 3");
                continue;
            }

            // minus one to translate from human indices to computer indices
            let player_move = Move {
                x: x - 1,
                y: y - 1,
            };

            let field_owner = board.get_field(&player_move);
            if field_owner != Player::Empty {
                println!("This field is already taken by {}", field_owner);
                continue;
            }

            return player_move;
        }
    }

    /// get a pseudo random move based on the system time
    fn get_random_move() -> Move {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(elapsed) => match elapsed.as_secs() % 4u64 {
                0 => Move::new(2, 2),
                1 => Move::new(2, 0),
                2 => Move::new(0, 2),
                _ => Move::new(0, 0),
            },
            // in case of an error just go with a default move
            Err(_) => Move::new(2, 2),
        }
    }
}

#[derive(Debug)]
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

/// one line on the games board
type Line = [Player; 3];

#[derive(Debug, Copy, Clone)]
struct Board {
    fields: [Line; 3]
}

impl Board {
    fn new () -> Board {
        Board{ fields: [[Player::Empty; 3]; 3] }
    }

    fn set_field(&mut self, pos: &Move, value: Player) {
        self.fields[pos.x][pos.y] = value;
    }

    fn get_field(&self, pos: &Move) -> Player {
        self.fields[pos.x][pos.y]
    }

    /// seach for 3 equal fields in one line.
    /// if there is a row, the function returns the "row owner".
    /// if there is not row, it returns Player::Empty.
    fn get_winner(&self) -> Player {
        let fields = self.fields;
        // test if 3 fields are equal
        fn line_owner(line: Line) -> Option<Player> {
            if line[0] != Player::Empty && line[0] == line[1] && line[1] == line[2] {
                return Some(line[0]);
            }
            return None;
        }

        for n in 0..3 {
            // test all columns
            if let Some(owner) = line_owner(fields[n]) {
                return owner;
            }

            // test all rows
            if let Some(owner) = line_owner([fields[0][n], fields[1][n], fields[2][n]]) {
                return owner;
            }
        }

        // test top left to bottom right
        if let Some(owner) = line_owner([fields[0][0], fields[1][1], fields[2][2]]) {
            return owner;
        }

        // test top right to bottom left
        if let Some(owner) = line_owner([fields[2][0], fields[1][1], fields[0][2]]) {
            return owner;
        }

        return Player::Empty;
    }

    /// check if no one is able to win in this game anymore aka it's a draw
    fn is_draw(&self) -> bool {
        let fields = self.fields;
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
            if is_possible_row(fields[n]) {
                return false;
            }

            // test row
            if is_possible_row([fields[0][n], fields[1][n], fields[2][n]]) {
                return false;
            }
        }

        // test top left to bottom right
        if is_possible_row([fields[0][0], fields[1][1], fields[2][2]]) {
            return false;
        }

        // test top right to bottom left
        if is_possible_row([fields[2][0], fields[1][1], fields[0][2]]) {
            return false;
        }

        return true;
    }

    /// write the board with ASCII characters to stdout and show the current game state
    fn display(&self) {
        // column indices
        println!("   A   B   C ");

        for y in 0..3 {
            if y != 0 {
                print!("  ---+---+---\n");
            }

            // row index
            print!("{} ", y + 1);

            for x in 0..3 {
                if x != 0 {
                    print!("|");
                }

                match self.fields[x][y] {
                    Player::Empty => print!("   "),
                    Player::Cross => print!(" X "),
                    Player::Circle => print!(" O "),
                }
            }
            println!("");
        }
    }
}

/// writes "[y/n]: " to stdout and waits for user input.
/// if the user enters "y" or "yes", the function returns true.
/// if the user enters "n" or "no", the function returns false.
/// if the user enters something else, the function informs the
/// user about the invaid input and repeats.
fn prompt_confirm() -> bool {
    loop {
        print!("[y/n]: ");

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
            _ => {
                println!("invalid input, enter \"y\" or \"n\"");
                print!("please try again: ");
                continue;
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut round_index = 0;
    let mut players = Players {
        current: Player::Circle,
        cross_is_ai: false,
        circle_is_ai: false,
    };
    let mut winner = Player::Empty;

    println!("Tic Tac Toe\n");

    print!("Should cross be controlled by the computer? ");
    players.cross_is_ai = prompt_confirm();
    print!("Should circle be controlled by the computer? ");
    players.circle_is_ai = prompt_confirm();

    if !(players.cross_is_ai && players.circle_is_ai) {
        println!("Make a move by entering the vertical position (A, B or C) and the horizontal position (1, 2 or 3)");
    }

    print!("\n");
    board.display();

    while winner == Player::Empty && !board.is_draw() {
        players.toggle();
        round_index += 1;

        println!("\n*****************");
        println!("Round:  {}", round_index);
        print!("Player: {} ", players.current);
        if players.current_is_ai() {
            print!(" (computer player)");
        }
        println!("");
        print!("Move:   ");

        let player_move;

        if round_index == 1 && players.current_is_ai() {
            player_move = Players::get_random_move();
        } else {
            player_move = players.get_next_move(&board)
        }

        if players.current_is_ai() {
            // show move
            print!("{} {}",
                match player_move.x {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    _ => "Error", // <- practically impossible
                },
                player_move.y+1
            );
        }

        // save the players move
        board.set_field(&player_move, players.current);
        // show the current game state
        print!("\n");
        board.display();
        // test if there are 3 equal fields in a row
        winner = board.get_winner();
    }

    if winner == Player::Empty {
        println!("Draw after {} rounds", round_index);
    } else if players.current_is_ai() {
        println!("The Computer ({}) won after {} rounds", winner, round_index);
    } else {
        println!("Player {} won after {} rounds", winner, round_index);
    }
}
