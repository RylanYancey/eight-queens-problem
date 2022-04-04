
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Piece {
    Queen { row: i32, col: i32 },
    Bishop { row: i32, col: i32 },
    Knight { row: i32, col: i32 },
    Rook { row: i32, col: i32 },
    King { row: i32, col: i32 },
    Pawn { row: i32, col: i32 },
}

impl Piece {
    pub fn attacks_square(&self, other_row: i32, other_col: i32) -> bool {
        match self {
            Piece::Queen {row, col} => {
                if *row == other_row || *col == other_col { return true }
                if row + col == other_row + other_col { return true } 
                if row - col == other_row - other_col { return true }
                false
            },
            Piece::Bishop { row, col } => {
                if row + col == other_row + other_col { return true } 
                if row - col == other_row - other_col { return true }
                false
            },
            Piece::Knight { row, col } => {
                if f32::sqrt(f32::powi((other_row - row) as f32, 2) + f32::powi((other_col - col) as f32, 2)) == 2.236068 { return true }
                if other_row == *row && other_col == *col { return true }
                false
            },
            Piece::Rook { row, col } => {
                *row == other_row || *col == other_col
            },
            Piece::King { row, col } => {
                i32::abs(*col - other_col) < 2 && i32::abs(*row - other_row) < 2
            }
            Piece::Pawn { row, col } => {
                if *row + 1 == other_row {
                    if *col + 1 == other_col || *col - 1 == other_col {
                        return true;
                    }
                }
                if *row == other_row && *col == other_col { return true }
                false
            }
        }
    }

    pub fn get_data(&self) -> (i32, i32) {

        match self {
            Piece::Queen { row, col } => (*row, *col),
            Piece::Bishop { row, col } => (*row, *col),
            Piece::Knight { row, col } => (*row, *col),
            Piece::Rook { row, col } => (*row, *col),
            Piece::King { row, col } => (*row, *col),
            Piece::Pawn { row, col } => (*row, *col),
        }

    }

    pub fn get_symbol(&self) -> (&str, char) {

        match self {
            Piece::Queen { row: _, col: _ } => ("Queen", 'Q'),
            Piece::Bishop { row: _, col: _ } => ("Bishop", 'B'),
            Piece::Knight { row: _, col: _ } => ("Knight", 'N',),
            Piece::Rook { row: _, col: _ } => ("Rook", 'R'),
            Piece::King { row: _, col: _ } => ("King", 'K'),
            Piece::Pawn { row: _, col: _ } => ("Pawn", 'p'),
        }

    }

    pub fn clone_with(&self, new_row: i32, new_col: i32) -> Piece {

        match self {
            Piece::Queen { row: _, col: _ } => {
                Piece::Queen { row: new_row, col: new_col }
            },
            Piece::Bishop { row: _, col: _ } => {
                Piece::Bishop { row: new_row, col: new_col }
            },
            Piece::Knight { row: _, col: _ } => {
                Piece::Knight { row: new_row, col: new_col }
            },
            Piece::Rook { row: _, col: _ } => {
                Piece::Rook { row: new_row, col: new_col }
            },
            Piece::King { row: _, col: _ } =>  {
                Piece::King { row: new_row, col: new_col }
            },
            Piece::Pawn { row: _, col: _ } => {
                Piece::Pawn { row: new_row, col: new_col }
            }    
        }

    }
}
