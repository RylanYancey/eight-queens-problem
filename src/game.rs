
use core::num;
use std::collections::BTreeSet;
use std::ops::Bound::{ Included, Excluded };

use super::Piece;
use super::overwrite;

extern crate itertools;
use itertools::Itertools;

extern crate rayon;
use rayon::prelude::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Game {
    solution: BTreeSet<Vec<Piece>>,
    bounds: Vec<Piece>,
    info: Vec<(i32, Piece)>,
    counter: i32,
    board_size: i32,
}

impl Game {
    pub fn new(bounds: Vec<(i32, Piece)>, board_size: i32) -> Self {

        if (board_size > 8) || (3 > board_size) { 
            panic!("board size cannot be greater than 8 or less than 3. If you want support for 9x9 you'll need to comment this panic out
                and implement 9x9 display. The display function can only display up to an 8x8 board.");
        }

        let mut bounds_2 = Vec::new();
        for (num, piece) in bounds.iter() {
            for _i in 0..*num {
                bounds_2.push(piece.clone());
            }
        }

        Game { 
            solution: BTreeSet::new(), 
            bounds: bounds_2, 
            info: bounds,
            counter: 0, 
            board_size: board_size 
        }
    }

    pub fn find(&mut self) {
        println!("");
        let mut valid: Vec<(i32, i32)> = Vec::new();
        for x in 1..=self.board_size {
            for y in 1..=self.board_size {
                valid.push((x, y));
            }
        }

        self.find_rec(Vec::new(), valid, 0);

        let mut message = "Finished after finding ".to_owned();
        message.push_str(self.counter.to_string().as_str());
        message.push_str(" solutions.");
        overwrite(message);
    }

    fn find_rec(&mut self, mut pieces: Vec<Piece>, valid: Vec<(i32, i32)>, step: i32) {
        if pieces.len() == self.bounds.len() {
            pieces.sort_unstable();
            if !self.solution.contains(&mut pieces) {
                self.solution.insert(pieces);
                self.counter += 1;
                overwrite(self.counter.to_string());
            }
        }
        else {
            let mut new_pieces: Vec<Piece>;
            'outer: for (x, y) in valid.iter() {

                let new_piece = self.bounds[step as usize].clone_with(*x, *y);

                for q in pieces.iter() {
                    let data = q.get_data();
                    if new_piece.attacks_square(data.0, data.1) { continue 'outer; }
                }

                let mut new_valid = valid.clone();

                let mut i: usize = 0;
                while i < new_valid.len() {

                    let location: &(i32, i32) = &new_valid[i];
                    if new_piece.attacks_square(location.0, location.1) {
                        new_valid.remove(i);
                    }
                    else {
                        i += 1;
                    }

                }

                new_pieces = pieces.clone();
                new_pieces.push(new_piece);

                if !((self.bounds.len() - new_pieces.len()) > new_valid.len()) {
                    self.find_rec(new_pieces, new_valid, step.clone() + 1);
                }
            }
        }
    }

    pub fn find_and_display(&mut self, target: i32, path: String) {
        println!("");
        let mut valid: Vec<(i32, i32)> = Vec::new();
        for x in 1..=self.board_size {
            for y in 1..=self.board_size {
                valid.push((x, y));
            }
        }

        self.find_rec_and_display(Vec::new(), valid, 0, target, &path);

        let mut message = "Finished after finding ".to_owned();
        message.push_str(self.counter.to_string().as_str());
        message.push_str(" solutions.");
        overwrite(message);
    }

    fn find_rec_and_display(&mut self, mut pieces: Vec<Piece>, valid: Vec<(i32, i32)>, step: i32, target: i32, path: &String) {
        if pieces.len() == self.bounds.len() {
            pieces.sort_unstable();
            if !self.solution.contains(&mut pieces) {
                self.solution.insert(pieces);
                self.counter += 1;
                overwrite(self.counter.to_string());
                if target == self.counter {
                    self.display(&path, target);
                    println!("{}", format!("Found all {}, terminating process.", target));
                    std::process::exit(1);
                }
            }
        }
        else {
            let mut new_pieces: Vec<Piece>;
            'outer: for (x, y) in valid.iter() {

                let new_piece = self.bounds[step as usize].clone_with(*x, *y);

                for q in pieces.iter() {
                    let data = q.get_data();
                    if new_piece.attacks_square(data.0, data.1) { continue 'outer; }
                }

                let mut new_valid = valid.clone();

                let mut i: usize = 0;
                while i < new_valid.len() {

                    let location: &(i32, i32) = &new_valid[i];
                    if new_piece.attacks_square(location.0, location.1) {
                        new_valid.remove(i);
                    }
                    else {
                        i += 1;
                    }

                }

                new_pieces = pieces.clone();
                new_pieces.push(new_piece);

                if !((self.bounds.len() - new_pieces.len()) > new_valid.len()) {
                    self.find_rec_and_display(new_pieces, new_valid, step.clone() + 1, target, path);
                }
            }
        }
    }

    pub fn display(&self, path: &str, num_of_solutions: i32) {
        // Preps our path and implements Display
        let path = Path::new(&path);
        let display = path.display();

        // Opens the file in write-only mode. Errors if need be.
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Generate display string that we will write to the file.
        let message = self.generate_display(num_of_solutions);    

        // Write the message to the file.
        match file.write_all(&message.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

    }

    fn generate_display(&self, mut num_of_solutions: i32) -> String {

        /*
          |  +---+---+---+---+---+---+---+---+
          |8 |   |   |   |   |   |   |   |   |
          |  +---+---+---+---+---+---+---+---+
          |7 |   |   |   |   |   |   |   |   |
          |  +---+---+---+---+---+---+---+---+
          |6 |   |   |   |   |   |   |   |   |
          |  +---+---+---+---+---+---+---+---+
          |5 |   |   |   |   |   |   |   |   |
          |  +---+---+---+---+---+---+---+---+
          |4 |   |   |   |   |   |   |   |   |
          |  +---+---+---+---+---+---+---+---+
          |3 |   |   |   |   |   |   |   |   |
          |  +---+---+---+---+---+---+---+---+
          |2 |   |   |   |   |   |   |   |   |
          |  +---+---+---+---+---+---+---+---+
          |1 |   |   |   |   |   |   |   |   |
          |  +---+---+---+---+---+---+---+---+
          |    a   b   c   d   e   f   g   h
          +-----------------------------------
        */

        let board = [
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            ['8','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|'],
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            ['7','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|'],
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            ['6','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|'],
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            ['5','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|'],
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            ['4','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|'],
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            ['3','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|'],
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            ['2','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|'],
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            ['1','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|',' ',' ',' ','|'],
            [' ','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+','-','-','-','+'],
            [' ',' ',' ','a',' ',' ',' ','b',' ',' ',' ','c',' ',' ',' ','d',' ',' ',' ','e',' ',' ',' ','f',' ',' ',' ','g',' ',' ',' ','h',' ',' '],
        ];

        let mut problem = String::new().to_owned();
        for (num, piece) in self.info.iter() {
            let symbol = piece.get_symbol();
            problem.push_str(format!("{} {} ", num, symbol.0).as_str());
        }
        problem.push_str(" problem");

        if num_of_solutions > self.counter || 0 > num_of_solutions { 
            num_of_solutions = self.counter; 
        }

        let mut message = format!(
            "\nThe solutions to the {} on a {} x {} board.\n The program found {} solutions, and {} of them are displayed here.\n\n",
            problem, self.board_size, self.board_size, self.counter, num_of_solutions
        );

        for n in self.solution.iter().take(num_of_solutions as usize) {
            let mut new_b = board.clone();
            for p in n.iter() {
                let mut location = p.get_data();
                location.0 = 18 - (location.0 * 2 + 1); 
                location.1 = (4 * location.1) - 1;
                new_b[location.0 as usize][location.1 as usize] = p.get_symbol().1;
            }

            for x in (16 - self.board_size * 2)..18 {
                message.push_str("\n");
                for y in 0..(self.board_size * 4) + 2 {
                    message.push(new_b[x as usize][y as usize]);
                }
            }
            message.push_str("\n\n");
        }

        message
    }

}