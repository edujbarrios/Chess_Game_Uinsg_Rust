
use std::collections::HashMap;

enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

struct Piece {
    piece_type: PieceType,
    color: Color,
}

enum Color {
    White,
    Black,
}

struct Square {
    row: i32,
    column: i32,
}

struct ChessBoard {
    squares: HashMap<Square, Option<Piece>>,
}

impl ChessBoard {
    fn new() -> ChessBoard {
        let mut board = ChessBoard {
            squares: HashMap::new(),
        };

        for row in 0..8 {
            for col in 0..8 {
                let square = Square { row, column: col };
                let piece = if row == 1 || row == 6 {
                    Some(Piece {
                        piece_type: PieceType::Pawn,
                        color: if row == 1 { Color::Black } else { Color::White },
                    })
                } else {
                    None
                };
                board.squares.insert(square, piece);
            }
        }

        board
    }

    fn move_piece(&mut self, from: Square, to: Square) {
        let piece = self.squares[&from].take();
        self.squares.insert(to, piece);
    }
}

fn main() {
    let mut board = ChessBoard::new();
    println!("Initial chess board:");
    println!("{:?}", board);

    let from = Square { row: 6, column: 4 };
    let to = Square { row: 4, column: 4 };
    board.move_piece(from, to);
    println!("After moving a pawn:");
    println!("{:?}", board);
}
