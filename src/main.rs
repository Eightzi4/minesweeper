use rand::Rng;
use std::io::{self, Write};
use std::str::FromStr;

const MINE: i8 = -1;

fn main() {
    let width: usize = get_input("Mine field width:\n");
    let height: usize = get_input("Mine field height:\n");
    let mine_count: usize = get_input("Mine count:\n");

    let (width, height) = get_valid_size(width, height);
    let mine_count = get_valid_mine_count(width, height, mine_count);

    println!(
        "Generating {}x{} mine field with {} mines:",
        width, height, mine_count
    );

    let mut mine_field = MineField::new(width, height, mine_count);
    mine_field.fill();
    mine_field.print();

    println!("Press enter to exit...");
    _ = io::stdin().read_line(&mut String::new());
}

struct MineField {
    field: Box<[Box<[i8]>]>,
    mine_count: usize,
}

impl MineField {
    fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let field = (0..height)
            .map(|_| vec![0i8; width].into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { field, mine_count }
    }

    fn fill(&mut self) {
        self.zero();
        let mut rng = rand::rng();
        let mut placed_mines = 0;

        let width = self.field[0].len();
        let height = self.field.len();

        while placed_mines < self.mine_count {
            let x = rng.random_range(0..width);
            let y = rng.random_range(0..height);

            if self.field[y][x] == MINE {
                continue;
            }

            self.field[y][x] = MINE;

            let start_y = y.saturating_sub(1);
            let end_y = (y + 1).min(height - 1);
            let start_x = x.saturating_sub(1);
            let end_x = (x + 1).min(width - 1);

            for y in start_y..=end_y {
                for x in start_x..=end_x {
                    if self.field[y][x] != MINE {
                        self.field[y][x] += 1;
                    }
                }
            }

            placed_mines += 1;
        }
    }

    fn zero(&mut self) {
        self.field.iter_mut().for_each(|row| row.fill(0));
    }

    fn print(&self) {
        fn print_horizontal_line(width: usize) {
            print!("+-");
            for _ in 0..width {
                print!("---");
            }
            println!("-+");
        }

        print_horizontal_line(self.field[0].len());

        for row in self.field.iter() {
            print!("|");
            for &cell in row.iter() {
                if cell == MINE {
                    print!(" {}", cell);
                } else {
                    print!("  {}", cell);
                }
            }
            println!("  |");
        }

        print_horizontal_line(self.field[0].len());
    }
}

fn get_input<T: FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(value) = input.trim().parse() {
            return value;
        }
        println!("Invalid input, please try again");
    }
}

fn get_valid_size(width: usize, height: usize) -> (usize, usize) {
    (
        if width > 6 { width } else { 9 },
        if height > 6 { height } else { 9 },
    )
}

fn get_valid_mine_count(width: usize, height: usize, mine_count: usize) -> usize {
    let max_mines = width * height / 10;
    if mine_count > 1 && mine_count <= max_mines {
        mine_count
    } else {
        max_mines
    }
}
