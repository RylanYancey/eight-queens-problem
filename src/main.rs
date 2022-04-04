
#![allow(unused_imports)]

pub mod game;
use game::Game;

pub mod piece;
use piece::{ Piece };

pub mod utility;
use utility::overwrite;

fn main() {

    let mut _game: Game = Game::new(vec![
            (1, Piece::Queen{row: 0, col: 0}),
            (2, Piece::Bishop{row: 0, col: 0}),
            (2, Piece::Knight{row: 0, col: 0}),
            (7, Piece::Pawn{row: 0, col: 0}),
            (2, Piece::Rook{row: 0, col: 0}),
            (1, Piece::King{row: 0, col: 0}),
            
        ], 8);


    let mut _game: Game = Game::new(vec![
            (1, Piece::Queen{row: 0, col: 0}),
            (1, Piece::King{row:0, col:0}),
            (2, Piece::Bishop{row:0, col:0}),
            (2, Piece::Knight{row:0, col:0}),
            (5, Piece::Pawn{row: 0, col:0}),
            (2, Piece::Rook{row:0, col:0}),
        ], 5);

    let mut game: Game = Game::new(vec![
            (5, Piece::Queen{row:0, col:0}),
            (2, Piece::Bishop{row:0, col:0}),
            (2, Piece::Knight{row:0, col:0}),
            (1, Piece::King{row:0, col:0}),
        ], 8);

    let mut _game: Game = Game::new(vec![
        (8, Piece::Queen{row:0, col:0}),
    ], 8);

    //game.find();
    //game.display("thing.txt", 30);

    game.find_and_display(10, "thing.txt".to_string());

}
