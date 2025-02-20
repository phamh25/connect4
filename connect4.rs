
use std::io::{stdin,stdout, Write};
use std::error::Error;
use std::fmt::{Display,Formatter};
use std::fmt;

/////////////////////////////////////////////
//
// Data structres used in Connect 4
//
/////////////////////////////////////////////

/// A Connect 4 piece
/// A piece can either be 
///  * R (player 1)
///  * B (player 2)
///  * E (empty)
#[derive(PartialEq, Clone, Copy)]
enum Piece { R, B, E }

use Piece::{R, B, E};

/// A Connect 4 board is a 2d vector of pieces
type Board = Vec<Vec<Piece>>;

// a move on the board, this is a pair of row and column
type Move = usize;

/////////////////////////////////////////////
//
// Code for printing out the board
//
/////////////////////////////////////////////

/// Prints out the Connect 4 board
/// The format of the board should be
///
///    1 2 3 4 5 6 7
///   +-------------+
/// A | | | | | | | |
///   |-+-+-+-+-+-+-|
/// B | | | | | | | |
///   |-+-+-+-+-+-+-|
/// C | | | | | | | |
///   |-+-+-+-+-+-+-|
/// D | | | | | | | |
///   |-+-+-+-+-+-+-|
/// E | | | | | | | |
///   |-+-+-+-+-+-+-|
/// F | | | | | | | |
///   +-------------+
///
/// # Arguments
/// 
/// * board - The board to print out
fn print_board(board : &Board)
{
    // Part 1: print out the board.
    // It should be formatted like the comment above.

    // column numbers and top board line
    println!("\n 1 2 3 4 5 6 7");
    println!("+-------------+");

    // prints board for every piece
    for i in 0..board.len() 
    {
        for &piece in &board[i]
        {
            match piece 
            {
                Piece::E => print!("| "),
                Piece::R => print!("|R"),
                Piece::B => print!("|B"),
            }
        }
        print!("|");

        // prints the last line
        if i == 5 
        {
            println!("\n+-------------+\n")
        }
        else 
        {
            println!("\n|-+-+-+-+-+-+-|");
        }
    }

}

/// create a new board that is a 7x6 vector with all empty squares.
fn new_board() -> Board
{
    vec![vec![E; 7]; 6]
}

/// creates a test board to test printing
///
/// print(test_board()) should print out
///    1 2 3 4 5 6 7
///   +-------------+
/// A | | | | | | | |
///   |-+-+-+-+-+-+-|
/// B | | | | | | | |
///   |-+-+-+-+-+-+-|
/// C |B|R|B|R|B|R|B|
///   |-+-+-+-+-+-+-|
/// D |B|R|B|R|B|R|B|
///   |-+-+-+-+-+-+-|
/// E |R|B|R|B|R|B|R|
///   |-+-+-+-+-+-+-|
/// F |R|B|R|B|R|B|R|
///   +-------------+
// fn test_board() -> Board
// {
//     vec![vec![E, E, E, E, E, E, E],
//          vec![E, E, E, E, E, E, E],
//          vec![B, R, B, R, B, R, B],
//          vec![B, R, B, R, B, R, B],
//          vec![R, B, R, B, R, B, R],
//          vec![R, B, R, B, R, B, R]]
// }


/////////////////////////////////////////////
//
// Main game logic
//
/////////////////////////////////////////////

/// runs the application
///  * set up the board
///  * run the game
///  * print the result
fn main()
{
    // use this to test out your print function
    // print_board(&test_board());

    // prints initial board
    print_board(&new_board());

    let mut board = new_board();
    match run_game(&mut board)
    {
        R => {println!("\n\nRed won!");}
        B => {println!("\n\nBlack won!");}
        E => {println!("\n\ndraw!");}
    }
    print_board(&board);
}

/// Runs the Connect 4 game, and returns the winning player.
///
/// A game is played with two players.
///  * For each turn we ask the player to take a turn, 
///    and update the board
///  * Next we check for a winner.
///  * If that player won, then we return their piece.
///  * If they didn't win, then play passes to the next player.
///
/// # Arguments
/// 
/// * board - the state of the board
fn run_game(board : &mut Board) -> Piece
{
    // Pert 2: write the code for running the game.
    // creates the current player
    let mut curr_player : i32 = 0;

    // loops while game is running
    while check_winner(board) == None 
    {
        // for player 1
        if curr_player == 0
        {
            human_turn(board, R);
            curr_player = 1; // switch players
        }
        // for player 2
        else
        {
            human_turn(board, B);
            curr_player = 0; // switch players
        }
    }

    // checks for a winner
    match check_winner(board)
    {
        Some(piece) => piece,
        None => E
    }
}


/// Try to make a move,
/// If the use fails to enter a valid move,
/// then print the error and ask the user again.
///
/// When the user enters a valid move,
/// then update the board,
/// and return if there was a winner.
fn human_turn(board : &mut Board, piece : Piece) -> Option<Piece>
{
    // Part 3: write code for a human players turn.
    // asks player for a move
    let player_move = try_move();

    match player_move
    {
        // if player move is valid
        Ok(c) => if board[0][c] == E
        {
            // place the piece
            for r in (0..6).rev()
            {
                if board[r][c] == E{
                    board[r][c] = piece;
                    break;
                }
            }
            print_board(&board); // updates board
        }

        // if player move is full
        else 
        {
            println!("Column is full.");
            human_turn(board, piece);
        },

        // if player move is not valid
        Err(_e) => {println!("Error: Please enter a valid number.");
            human_turn(board, piece);},
        };

        // checks for winner
        return check_winner(board);
}

/// check to see if there's a winner
/// If there is a winner, then return that player's piece.
/// If the game is a draw, then return E.
/// Otherwise return None.
///
/// # Arguments
///
/// * board - the state of the board
fn check_winner(board : &Board) -> Option<Piece>
{
    //Part 4: check to see if the board has a winner.
    //If there is a winner return Some(Piece).
    //
    //If there's a draw return Some(E).
    //
    //If there's no winner yet return None
    //

     // checks for horizontal
     for r in 0..6
     {
         for c in 0..4
         {
             if board[r][c] != Piece::E
             && board[r][c] == board[r][c + 1]
             && board[r][c] == board[r][c + 2]
             && board[r][c] == board[r][c + 3]
             {
                 return Some(board[r][c]);
             }
         }
     }
 
     // checks for vertical
     for r in 0..3 
     {
         for c in 0..7
         {
             if board[r][c] != Piece::E
             && board[r][c] == board[r + 1][c]
             && board[r][c] == board[r + 2][c]
             && board[r][c] == board[r + 3][c]
             {
                 return Some(board[r][c]);
             }
         }
     }
 
     // checks for diagonal (case 1)
     for r in 0..3
     {
         for c in 0..4 
         {
             if board[r][c] != Piece::E
             && board[r][c] == board[r + 1][c + 1]
             && board[r][c] == board[r + 2][c + 2]
             && board[r][c] == board[r + 3][c + 3]
             {
                 return Some(board[r][c]);
             }
         }
     }
 
     // checks for diagonal (case 2)
     for r in 3..6 
     {
         for c in 0..4
         {
             if board[r][c] != Piece::E
             && board[r][c] == board[r - 1][c + 1]
             && board[r][c] == board[r - 2][c + 2]
             && board[r][c] == board[r - 3][c + 3]
             {
                 return Some(board[r][c]);
             }
         }
     }
 
     // check for a draw
     let mut draw:bool = true;
     for c in 0..7 
     {
         if board[0][c] == E 
         {
             draw = false;
         }
     }
 
     if draw {return Some(E)}
     return None;
}


/////////////////////////////////////////////
//
// Code to ask for, and validate, a human player's move
//
/////////////////////////////////////////////

/// Ask the user to input a move,
/// and check that the move is valid.
/// Return the move if it's valid.
/// Return a GameError on faliure.
///
/// # Arguments
/// 
/// * board - the current State of the board
fn try_move() -> Result<Move, GameError>
{
    let line = ask_for_move()?;
    return validate_move(&line);
}

/// Ask the user for a move, retun the move is possible
fn ask_for_move() -> Result<String, GameError>
{
    print!("Select a square? ");
    stdout().flush().unwrap();
    let mut line = String::new();
    match stdin().read_line(&mut line)
    {
        Ok(_)  => {return Ok(line.trim().to_string());}
        Err(e) => {return Err(game_error(&e.to_string()));}
    }
}

/// Validate and parse the move
/// A move must be a single integer between 1 and 7
/// and C is one of 1, 2, 3, 4, 5, 6, 7
///
/// This will return OK(n) if n is a valid move.
/// Note this does not garuentee that the column has
/// space for a move, you need to do that yourself.
///
/// # Arguments
///
/// * board - the state of the board
///
fn validate_move(col : &str) -> Result<Move, GameError>
{
    let c = match col
            {
                "1" => Ok(0), 
                "2" => Ok(1), 
                "3" => Ok(2), 
                "4" => Ok(3), 
                "5" => Ok(4), 
                "6" => Ok(5), 
                "7" => Ok(6), 
                _   => Err(game_error("column must be one of 1-7"))
            }?;
    return Ok(c);
}


//////////////////////////////////////////////
//
// Game Error code: You don't need to worry about this.
//
// you can print a GameError (ge) with
// println!("{}", ge);
//
//////////////////////////////////////////////

/// A GameError is an error with Tic Tac Toe
#[derive(Debug, Clone)]
struct GameError { msg : String }

/// make a new game error
fn game_error(err : &str) -> GameError
{
    GameError{msg: err.to_string()}
}

impl Display for GameError
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f,"Game Error: {}",self.msg)
    }
}

impl Error for GameError
{
    fn description(&self) -> &str
    {
        &self.msg
    }
}
