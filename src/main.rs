#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<u8>>,
}

impl Board {
    fn build(width: usize, height: usize) -> Board {
        let mut cells = vec![vec![0; width]; height];
        cells.insert(0, vec![2; width]);
        cells.push(vec![2; width]);
        for columns in &mut cells {
            columns.insert(0, 2);
            columns.push(2);
        }
        let board = Board { width, height, cells };
        board
    }

    fn copy(&self) -> Board {
        Board { width: self.width, height: self.height, cells: self.cells.to_vec() }
    }

    fn set_value(&mut self, value: u8, x: usize, y: usize) {
        self.cells[y+1][x+1] = value;
    }

    fn get_value(&self, x: usize, y: usize) -> u8 {
        self.cells[y+1][x+1]
    }

    fn count_alive_adjacent_cells(&self, x: usize, y: usize) -> u32 {
        let mut counter = 0;
        if self.cells[y][x] == 1 { counter += 1; };
        if self.cells[y+1][x] == 1 { counter += 1; };
        if self.cells[y+2][x] == 1 { counter += 1; };
        if self.cells[y][x+1] == 1 { counter += 1; };
        if self.cells[y+2][x+1] == 1 { counter += 1; };
        if self.cells[y][x+2] == 1 { counter += 1; };
        if self.cells[y+1][x+2] == 1 { counter += 1; };
        if self.cells[y+2][x+2] == 1 { counter += 1; };
        counter
    }

    fn print(&self) {
        let cells_iter = self.cells.iter();

        for columns in cells_iter {
            let row = columns.iter()
                .map(|&column| {
                    if column >= 2 { String::from("") } else { column.to_string() }
                })
                .collect::<Vec<String>>()
                .join("");

            println!("{}", row);
        }
    }
}

#[derive(Debug)]
struct Game {
    board: Board,
}

impl Game {
    fn build() -> Game {
        let board = Board::build(20, 20);
        let game = Game { board };
        game
    }

    fn run(&mut self) {
        self.board.print();

        for _ in 0..10 {
            self.next_turn();
            self.board.print();
        }
    }

    fn set_glider(&mut self, x: usize, y: usize) {
        self.board.set_value(0, x, y);
        self.board.set_value(0, x, y+1);
        self.board.set_value(1, x, y+2);
        self.board.set_value(1, x+1, y);
        self.board.set_value(0, x+1, y+1);
        self.board.set_value(1, x+1, y+2);
        self.board.set_value(0, x+2, y);
        self.board.set_value(1, x+2, y+1);
        self.board.set_value(1, x+2, y+2);
    }

    fn next_turn(&mut self) {
        let mut next_board = self.board.copy();

        for y in 0..self.board.height {
            for x in 0..self.board.width {
                let current_cell = self.board.get_value(x, y);
                let alive_count = self.board.count_alive_adjacent_cells(x, y);

                let is_alive = if current_cell == 0 {
                    alive_count == 3
                } else {
                    alive_count == 2 || alive_count == 3
                };

                let next_value = if is_alive { 1 } else { 0 };
                next_board.set_value(next_value, x, y);
            }
        }

        self.board = next_board;
    }
}

fn main() {
    let mut game = Game::build();
    game.set_glider(1, 1);
    game.run();
}
