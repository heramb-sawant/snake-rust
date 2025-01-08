use rand::{thread_rng, Rng};
use termion::clear;
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

use std::io::{stdin, stdout, Stdin, Stdout, Write};

/*
    TODO:
        - Randomly generate monsters on the board
        - Get display using termion
        - Get movements working
        - A few tests

        - See if you can get GH tests pipeline working
*/

#[derive(Clone)]
enum Tile {
    Empty,
    Player,
    Enemy,
    Wall,
    Exit,
}

impl Tile {
    fn get_character(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Player => 'P',
            Tile::Enemy => 'E',
            Tile::Wall => '+',
            Tile::Exit => 'O',
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    columns: usize,
    rows: usize,
    // TODO: move this to the tiles as well. Update this with a player tile once coordinate is on the player
    player_coordinate: (usize, usize),
    stdout: RawTerminal<Stdout>,
    stdin: Keys<Stdin>,
}

impl Grid {
    fn new(columns: usize, rows: usize, stdout: RawTerminal<Stdout>, stdin: Stdin) -> Grid {
        let mut empty_grid = vec![vec![Tile::Empty; rows]; columns];

        // Add walls
        let mut i = 0;
        while i < rows {
            empty_grid[0][i] = Tile::Wall;
            empty_grid[rows - 1][i] = Tile::Wall;
            i += 1;
        }

        // Add walls
        let mut i = 0;
        while i < columns {
            empty_grid[i][0] = Tile::Wall;
            empty_grid[i][columns - 1] = Tile::Wall;
            i += 1;
        }

        // Add Player
        let player_coordinate = (1, 1);
        empty_grid[1][1] = Tile::Player;

        // Add Exit
        empty_grid[columns - 2][rows - 2] = Tile::Exit;

        // Add Enemy
        let enemy_column: usize = thread_rng().gen_range(2..18);
        let enemy_row: usize = thread_rng().gen_range(2..18);

        empty_grid[enemy_column][enemy_row] = Tile::Enemy;

        return Grid {
            tiles: empty_grid,
            columns,
            player_coordinate,
            rows,
            stdout,
            stdin: stdin.keys(),
        };
    }

    fn start(&mut self) {
        // TODO: This clears some of the warns I think. Make sure it starts after all the console start output
        write!(self.stdout, "{}", clear::All).unwrap();
        // self.stdout.flush().unwrap();

        self.print();

        writeln!(
            self.stdout,
            "{}Arrow keys to move, q to exit.",
            termion::cursor::Goto(0, 21),
        )
        .unwrap();

        loop {
            let c = self.stdin.next().unwrap().unwrap();
            // Clear the current line.
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(0, 22),
                termion::clear::CurrentLine
            )
            .unwrap();

            match c {
                // Exit.
                Key::Char('q') => break,
                Key::Left => {
                    println!("<left>");
                    self.move_player(Direction::Left)
                }
                Key::Right => {
                    println!("<right>");
                    self.move_player(Direction::Right);
                }
                Key::Up => {
                    println!("<up>");
                    self.move_player(Direction::Up);
                }
                Key::Down => {
                    println!("<down>");
                    self.move_player(Direction::Down);
                }
                _ => println!("Invalid Move"),
            }

            self.stdout.flush().unwrap();
        }

        // Show the cursor again before we exit.
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }

    fn move_player(&mut self, direction: Direction) {
        let old_coordinate = self.player_coordinate;

        let new_coordinate = match direction {
            Direction::Right => (old_coordinate.0 + 1, old_coordinate.1),
            Direction::Left => (old_coordinate.0 - 1, old_coordinate.1),
            Direction::Up => (old_coordinate.0, old_coordinate.1 - 1),
            Direction::Down => (old_coordinate.0, old_coordinate.1 + 1),
        };

        writeln!(
            self.stdout,
            "{}Old {:?}, New {:?}",
            termion::cursor::Goto(0, 24),
            old_coordinate,
            new_coordinate
        )
        .unwrap();

        // TODO: Use the actual tile character instead of hard coding it
        write!(
            self.stdout,
            "{}P",
            termion::cursor::Goto(new_coordinate.0 as u16 + 1, new_coordinate.1 as u16 + 1), // TODO: Find a better way around this cast
        )
        .unwrap();
        write!(
            self.stdout,
            "{} {}",
            termion::cursor::Goto(old_coordinate.0 as u16 + 1, old_coordinate.1 as u16 + 1), // TODO: Find a better way around this cast
            termion::cursor::Hide
        )
        .unwrap();

        self.player_coordinate = new_coordinate;
    }

    // TODO: get rid of this mut with pointers or whatever you saw in the tutorial.
    fn print(&mut self) {
        // TODO: Update all the tiles to have a coordinate so its easier to reference and print them at a specific position
        let mut i = 1;
        for columns in self.tiles.iter() {
            write!(self.stdout, "{}", termion::cursor::Goto(0, i)).unwrap();
            i += 1;
            for row in columns.iter() {
                write!(self.stdout, "{}", row.get_character()).unwrap()
            }
            writeln!(self.stdout, "").unwrap()
        }
    }
}

fn main() {
    println!("Starting!");

    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();

    // TODO: Get grid size from input.
    let length = 20;
    let mut grid = Grid::new(length, length, stdout, stdin);

    grid.start();
}
