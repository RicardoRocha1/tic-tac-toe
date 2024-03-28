use std::io;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Empty,
    X,
    O,
}

struct Board {
    cells: [[Cell; 3]; 3],
}


impl Board {
    fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    fn display(&self) {
        for row in &self.cells {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => " ",
                    Cell::X => "X",
                    Cell::O => "O",
                };
                print!(" {} ", symbol);
            }
            println!();
        }
    }

    fn make_move(&mut self, row: usize, col: usize, player: Cell) -> Result<(), String> {
        if row >= 3 || col >= 3 {
            return Err(String::from("Invalid move: Row and column out of range."));
        }
        if self.cells[row][col] != Cell::Empty {
            return Err(String::from("Invalid move: Cell already occupied."));
        }

        self.cells[row][col] = player;
        Ok(())
    }

    fn check_winner(&self) -> Option<Cell> {
        for i in 0..3 {
            if self.cells[i][0] != Cell::Empty
                && self.cells[i][0] == self.cells[i][1]
                && self.cells[i][0] == self.cells[i][2]
            {
                return Some(self.cells[i][0]);
            }
            if self.cells[0][i] != Cell::Empty
                && self.cells[0][i] == self.cells[1][i]
                && self.cells[0][i] == self.cells[2][i]
            {
                return Some(self.cells[0][i]);
            }
        }

        if self.cells[0][0] != Cell::Empty
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[0][0] == self.cells[2][2]
        {
            return Some(self.cells[0][0]);
        }

        if self.cells[0][2] != Cell::Empty
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[0][2] == self.cells[2][0]
        {
            return Some(self.cells[0][2]);
        }

        None
    }
}

fn main() {
    let mut board = Board::new();
    let mut player = Cell::X;

    println!("Welcome to Tic-Tac-Toe!");

    loop {
        println!("\nCurrent board:");
        board.display();

        println!("\nPlayer {:?}'s turn.", player);

        loop {
            println!("Enter row (0-2):");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            let row: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number between 0 and 2.");
                    continue;
                }
            };

            println!("Enter column (0-2):");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            let col: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number between 0 and 2.");
                    continue;
                }
            };

            match board.make_move(row, col, player) {
                Ok(_) => break,
                Err(msg) => {
                    println!("{}", msg);
                    continue;
                }
            }
        }

        if let Some(winner) = board.check_winner() {
            println!("\nPlayer {:?} wins!", winner);
            break;
        } else if !board.cells.iter().any(|row| row.contains(&Cell::Empty)) {
            println!("\nIt's a draw!");
            break;
        }

        player = match player {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            Cell::Empty => Cell::Empty,
        };
    }
}
