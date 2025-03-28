use rand::Rng;
use std::io::{self, Write};
use std::str::FromStr;

// Game cell state constants
const REVEALED_MINE: i8 = 9;        // A revealed mine (game over)
const UNREVEALED_MINE: i8 = -9;     // A hidden mine
const REVEALED_EMPTY: i8 = 127;     // A revealed empty cell with no adjacent mines
const UNREVEALED_EMPTY: i8 = 0;     // A hidden empty cell
const MAX_SIZE: usize = 99;         // Maximum allowed size for the game board (so that the board formatting doesn't break)

fn main() {
    // Get game parameters from user
    let (width, height) = get_input_vec2("Mine field size (width height): ", MAX_SIZE, MAX_SIZE);
    let mine_count: usize = get_input("Mine count: ");

    // Validate and adjust parameters if needed
    let (width, height) = get_valid_size(width, height);
    let mine_count = get_valid_mine_count(width, height, mine_count);

    println!(
        "Generating {}x{} mine field with {} mines:",
        width, height, mine_count
    );

    // Initialize and set up the game
    let mut mine_field = MineField::new(width, height, mine_count);
    mine_field.fill();
    mine_field.print();

    // Main game loop
    loop {
        let (x, y) = get_input_vec2("Reveal coordinates (x y): ", width, height);
        
        // Convert from 1-based user coordinates to 0-based internal coordinates
        if mine_field.reveal(x - 1, y - 1) {
            println!("Game over! You hit a mine!");
            break;
        }
        mine_field.print();
    }

    // TODO: Allow exiting the game early and replaying, finishing the game, add colors
}

/// Represents the minesweeper game board
struct MineField {
    field: Box<[Box<[i8]>]>,  // 2D array of cell values
    mine_count: usize,        // Total number of mines on the board
}

impl MineField {
    /// Creates a new empty mine field with the specified dimensions
    fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let field = (0..height)
            .map(|_| vec![0i8; width].into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { field, mine_count }
    }

    /// Fills the mine field with mines and calculates adjacent mine counts
    fn fill(&mut self) {
        self.zero();
        let mut rng = rand::rng();
        let mut placed_mines = 0;

        let width = self.field[0].len();
        let height = self.field.len();

        // Place mines randomly
        while placed_mines < self.mine_count {
            let x = rng.random_range(0..width);
            let y = rng.random_range(0..height);

            // Skip if there's already a mine here
            if self.field[y][x] == UNREVEALED_MINE {
                continue;
            }

            // Place a mine
            self.field[y][x] = UNREVEALED_MINE;

            // Calculate bounds for adjacent cells
            let start_y = y.saturating_sub(1);
            let end_y = (y + 1).min(height - 1);
            let start_x = x.saturating_sub(1);
            let end_x = (x + 1).min(width - 1);

            // Update adjacent cell counts
            for y in start_y..=end_y {
                for x in start_x..=end_x {
                    if self.field[y][x] != UNREVEALED_MINE {
                        self.field[y][x] -= 1;
                    }
                }
            }

            placed_mines += 1;
        }
    }

    /// Reveals a cell at the given coordinates
    /// Returns true if a mine was revealed (game over), false otherwise
    fn reveal(&mut self, x: usize, y: usize) -> bool {
        match self.field[y][x] {
            // Empty cell - reveal it and all adjacent empty cells
            UNREVEALED_EMPTY => {
                self.field[y][x] = REVEALED_EMPTY;
                self.reveal_adjacent(x, y);
                false
            }
            // Mine - game over
            UNREVEALED_MINE => {
                self.field[y][x] = REVEALED_MINE;
                true
            }
            // Number cell - just reveal it
            _ => {
                self.field[y][x] = self.field[y][x].abs();
                false
            }
        }
    }

    /// Recursively reveals adjacent cells when an empty cell is revealed
    fn reveal_adjacent(&mut self, x: usize, y: usize) {
        let width = self.field[0].len();
        let height = self.field.len();

        // Calculate bounds for adjacent cells
        let start_y = y.saturating_sub(1);
        let end_y = (y + 1).min(height - 1);
        let start_x = x.saturating_sub(1);
        let end_x = (x + 1).min(width - 1);
        
        // Check all adjacent cells
        for dy in start_y..=end_y {
            for dx in start_x..=end_x {
                // Skip the current cell
                if dx == x && dy == y {
                    continue;
                }
                
                // Reveal adjacent cells if they're within bounds
                if dx < width && dy < height {
                    self.reveal(dx, dy);
                }
            }
        }
    }

    /// Resets all cells to empty
    fn zero(&mut self) {
        for row in self.field.iter_mut() {
            row.fill(0);
        }
    }

    /// Displays the current state of the mine field
    fn print(&self) {
        let width = self.field[0].len();
        
        // Print top coordinates
        print!("    ");
        for x in 1..=width {
            print!("{:2} ", x);
        }
        println!();
        
        // Print top border
        print!("  +");
        for _ in 0..width {
            print!("---");
        }
        println!("+");

        // Print each row with its y-coordinate
        for (y, row) in self.field.iter().enumerate() {
            print!("{:2}|", y + 1);
            for &cell in row.iter() {
                match cell {
                    REVEALED_EMPTY => print!("  ."),
                    n if n > 0 => print!(" {:2}", n),
                    _ => print!("   "),
                }
            }
            println!("|{:2}", y + 1);
        }

        // Print bottom border
        print!("  +");
        for _ in 0..width {
            print!("---");
        }
        println!("+");

        // Print bottom coordinates
        print!("    ");
        for x in 1..=width {
            print!("{:2} ", x);
        }
        println!();
    }
}

/// Gets user input and parses it to the specified type
fn get_input<T: FromStr>(prompt: &str) -> T 
where
    <T as FromStr>::Err: std::fmt::Debug,
{
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

/// Gets a pair of coordinates from the user
fn get_input_vec2(prompt: &str, max_x: usize, max_y: usize) -> (usize, usize) {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Please enter two numbers separated by space");
            continue;
        }

        if let (Ok(x), Ok(y)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            if x > 0 && x <= max_x && y > 0 && y <= max_y {
                return (x, y);
            }
            println!("Numbers must be in range: (1..{}) (1..{})", max_x, max_y);
        } else {
            println!("Invalid input, please enter two numbers");
        }
    }
}

/// Ensures the game board is at least the minimum size
fn get_valid_size(width: usize, height: usize) -> (usize, usize) {
    (
        if width > 6 { width } else { 9 },
        if height > 6 { height } else { 9 },
    )
}

/// Ensures the mine count is reasonable for the board size
fn get_valid_mine_count(width: usize, height: usize, mine_count: usize) -> usize {
    mine_count.clamp(2, width * height / 10)
}
