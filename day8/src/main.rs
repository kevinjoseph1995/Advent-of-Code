use std::fmt::Display;

struct Grid<T> {
    data: Vec<T>,
    num_rows: usize,
    num_cols: usize,
}

impl<T> Grid<T>
where
    T: Clone + Copy + Default,
{
    fn new() -> Self {
        Grid {
            data: vec![],
            num_rows: 0,
            num_cols: 0,
        }
    }
    fn construct_with_size(num_rows: usize, num_cols: usize) -> Self {
        Grid {
            data: vec![T::default(); num_cols * num_rows],
            num_rows,
            num_cols,
        }
    }
    fn at(&self, row: usize, col: usize) -> T {
        let index = (row as usize) * self.num_cols + (col as usize);
        return self.data[index];
    }
    fn update(&mut self, row: usize, col: usize, new_value: T) {
        let index = row * self.num_cols + col;
        self.data[index] = new_value;
    }
}

impl Grid<(u8, u8)> {
    fn display(&self) {
        for row in 0..self.num_rows {
            for val in self.data[row * self.num_cols..(row * self.num_cols + self.num_cols)].iter()
            {
                print!("({},{}) ", val.0, val.1);
            }
            println!("");
        }
    }
}

impl Grid<u8> {
    fn add_row(&mut self, row: &str) {
        self.num_cols = row.len();
        self.num_rows += 1;
        for ch in row.chars() {
            self.data.push(ch.to_digit(10).unwrap() as u8);
        }
    }
    fn display(&self) {
        for row in 0..self.num_rows {
            for val in self.data[row * self.num_cols..(row * self.num_cols + self.num_cols)].iter()
            {
                print!("{} ", val)
            }
            println!("");
        }
    }
}

fn problem_1(input: &str) {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.add_row(line);
    }
    let mut boundary_1 =
        Grid::<(u8, u8)>::construct_with_size(grid.num_rows - 2, grid.num_cols - 2);
    for row in 0..(grid.num_rows - 2) {
        for col in 0..(grid.num_cols - 2) {
            let mut new_value: (u8, u8) = (0, 0);
            // Update top boundary
            if row == 0 {
                new_value.1 = grid.at(row, col + 1);
            } else {
                new_value.1 = std::cmp::max(grid.at(row, col + 1), boundary_1.at(row - 1, col).1);
            }
            // Update left boundary
            if col == 0 {
                new_value.0 = grid.at(row + 1, col);
            } else {
                new_value.0 = std::cmp::max(grid.at(row + 1, col), boundary_1.at(row, col - 1).0);
            }
            boundary_1.update(row, col, new_value);
        }
    }
    let mut boundary_2 =
        Grid::<(u8, u8)>::construct_with_size(grid.num_rows - 2, grid.num_cols - 2);
    for row in (0..(grid.num_rows - 2)).rev() {
        for col in (0..(grid.num_cols - 2)).rev() {
            let mut new_value: (u8, u8) = (0, 0);
            // Update bottom boundary
            if row == (grid.num_rows - 3) {
                new_value.1 = grid.at(row + 2, col + 1);
            } else {
                new_value.1 =
                    std::cmp::max(grid.at(row + 2, col + 1), boundary_2.at(row + 1, col).1);
            }
            // Update right boundary
            if col == (grid.num_cols - 3) {
                new_value.0 = grid.at(row + 1, col + 2);
            } else {
                new_value.0 =
                    std::cmp::max(grid.at(row + 1, col + 2), boundary_2.at(row, col + 1).0);
            }
            boundary_2.update(row, col, new_value);
        }
    }
    let mut count = 0;
    for row in 1..=(grid.num_rows - 2) {
        for col in 1..=(grid.num_cols - 2) {
            if grid.at(row, col) > boundary_1.at(row - 1, col - 1).0
                || grid.at(row, col) > boundary_1.at(row - 1, col - 1).1
                || grid.at(row, col) > boundary_2.at(row - 1, col - 1).0
                || grid.at(row, col) > boundary_2.at(row - 1, col - 1).1
            {
                count += 1;
            }
        }
    }
    println!("{}", count + (2 * grid.num_rows + 2 * grid.num_cols - 4));
}

fn problem_2(input: &str) {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.add_row(line);
    }

    let mut max = 0;
    for row in 1..=(grid.num_rows - 2) {
        for col in 1..=(grid.num_cols - 2) {
            let mut array = [0, 0, 0, 0];
            // Left
            for i in (0..col).rev() {
                array[0] += 1;
                if grid.at(row, i) >= grid.at(row, col) {
                    break;
                }
            }
            // Right
            for i in (col + 1)..(grid.num_cols) {
                array[1] += 1;
                if grid.at(row, i) >= grid.at(row, col) {
                    break;
                }
            }
            //Up
            for i in (0..row).rev() {
                array[2] += 1;
                if grid.at(i, col) >= grid.at(row, col) {
                    break;
                }
            }
            // Down
            for i in (row + 1)..(grid.num_rows) {
                array[3] += 1;
                if grid.at(i, col) >= grid.at(row, col) {
                    break;
                }
            }
            println!("{:?} {:?}", (row, col), array);
            let product = array.into_iter().reduce(|a, x| a * x).unwrap();
            if product > max {
                max = product;
            }
        }
    }
    println!("{}", max);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("day1 <PROBLEM_NUMBER> <INPUT_FILE>");
        return;
    }
    let problem;
    match args[1].as_str() {
        "1" | "2" => problem = String::from(&args[1]),
        _ => {
            return;
        }
    }

    let input_filename = &args[2];
    let input = std::fs::read_to_string(input_filename);
    if let Err(error) = input {
        eprintln!("{}", error);
        return;
    }
    let input = input.unwrap();
    if problem == "1" {
        problem_1(&input);
    } else {
        problem_2(&input);
    }
}
