struct Board {
    square: [[bool; 8]; 8],
    queen_number: i8
}

impl Board {
    fn new(queens: &[(usize, usize)]) -> Board {
        let mut board = Board {
            square: [[false; 8]; 8],
            queen_number: 0
        };

        for &(i, j) in queens {
            board.square[i][j] = true;
            board.queen_number += 1;
        }
        return board;
    }

    fn display(&self) {
        for row in self.square.into_iter() {
            let mut line = String::from("");
            for square in row.iter() {
                match square {
                    &true => line += "Q ",
                    &false => line += ". "
                }
            }
            println!("{}", line);
        }
    }
}

fn main() {
    let queens : Vec<(usize, usize)> = vec![(1, 1), (2, 2)];
    let board = Board::new(&queens);
    board.display();
}
