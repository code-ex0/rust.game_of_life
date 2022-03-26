use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Matrix {
    fn new(width: usize, height: usize) -> Self {
        Matrix {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[self.get_index(x, y)]
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        let index = self.get_index(x, y);
        self.cells[index] = cell;
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in x-1..x+2 {
            for j in y-1..y+2 {
                if i == x && j == y {
                    continue;
                }
                if i < self.width && j < self.height {
                    match self.get_cell(i, j) {
                        Cell::Alive => count += 1,
                        Cell::Dead => count += 0,
                    }
                }
            }
        }
        count
    }

    fn next_gen(&self) -> Self {
        let mut next = Matrix::new(self.width, self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                let neighbours = self.count_neighbours(i, j);
                match self.get_cell(i, j) {
                    Cell::Alive => {
                        match neighbours {
                            2 | 3 => next.set_cell(i, j, Cell::Alive),
                            _ => next.set_cell(i, j, Cell::Dead),
                        }
                    },
                    Cell::Dead => {
                        match neighbours {
                            3 => next.set_cell(i, j, Cell::Alive),
                            _ => next.set_cell(i, j, Cell::Dead),
                        }
                    },
                }
            }
        }
        next
    }

    fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                match self.get_cell(j, i) {
                    Cell::Alive => print!("*"),
                    Cell::Dead => print!(" "),
                }
            }
            println!("");
        }
    }

    fn fill_randomly(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let r = rand::thread_rng().gen_range(0..2);
                match r {
                    0 => self.set_cell(i, j, Cell::Dead),
                    _ => self.set_cell(i, j, Cell::Alive),
                }
            }
        }
    }


}

fn main() {
    let mut matrix = Matrix::new(10, 10);
    matrix.fill_randomly();

    println!("welcome to the game of life");
    loop {
        matrix.print();
        if matrix.cells.iter().all(|c| *c == Cell::Dead) {
            println!("All cells are dead");
            break;
        }
        println!("");
        matrix = matrix.next_gen();
        std::thread::sleep(std::time::Duration::from_millis(500));

    }
}