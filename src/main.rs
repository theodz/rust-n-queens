use std::cmp::max;
use std::cmp::min;

struct Board {
    width: usize,
    height: usize,
    queens: Vec<(usize, usize)>
}

impl Board {
    fn display(&self) {
        let mut squares = vec![vec![false; self.height]; self.width];

        for queen in &self.queens {
            let &(qi, qj) = queen;
            squares[qi][qj] = true;
        }

        for row in squares {
            let mut line = String::from("");
            for square in row {
                match square {
                    true => line += "Q ",
                    false => line += ". "
                }
            }
            println!("{}", line);
        }
    }
}

fn incorrect(board : &Board) -> bool {
    for queen in &board.queens {
        let &(qi, qj) = queen;
        for other_queen in &board.queens {
            let &(oi, oj) = other_queen;
            let h_diff = max(oi, qi) - min(oi, qi);
            let v_diff = max(oj, qj) - min(oj, qj);

            if oi == qi && oj == qj {
                continue;
            } else if oi == qi || oj == qj || h_diff == v_diff {
                return true;
            }
        }
    }

    return false;
}

fn correct(board : &Board, queen_number: usize) -> bool {
    if board.queens.len() == queen_number {
        return true;
    } else {
        return false;
    }
}

fn followup(board : &Board) -> Vec<Board> {
    return vec![];
}

fn backtrack(board : &Board) {

}

fn main() {
    let queens = vec![(1, 1), (2, 3)];
    let board = Board { width: 8, height: 8, queens: queens };
    board.display();
}
