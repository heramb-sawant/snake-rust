use rand::{thread_rng, Rng};
use termion::clear;
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

use std::io::{stdin, stdout, Stdin, Stdout, Write};

/*
    TODO:
        - Print the individual pieces once the board works as intended
        - Snake moves on its own
        - Snake loses if it hits a border
        - Snake increases in size everytime you get food
        - Speed increases
        - Food animation
        - A few tests

        - See if you can get GH tests pipeline working
*/

#[derive(Clone)]
enum Tile {
    Empty,
    Snake,
    Food,
    Wall,
}

impl Tile {
    fn get_character(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Snake => '⍥',
            Tile::Food => 'o',
            Tile::Wall => '|',
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
    score: i16,
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
        empty_grid[1][1] = Tile::Snake;

        return Grid {
            tiles: empty_grid,
            columns,
            player_coordinate,
            rows,
            score: 0,
            stdout,
            stdin: stdin.keys(),
        };
    }

    fn start(&mut self) {
        // Set up game
        self.place_food();

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

    fn place_food(&mut self) {
        let food_column: usize = thread_rng().gen_range(2..self.columns - 2);
        let food_row: usize = thread_rng().gen_range(2..self.rows - 2);

        self.tiles[food_column][food_row] = Tile::Food;
    }

    fn move_player(&mut self, direction: Direction) {
        let old_coordinate = self.player_coordinate;

        let new_coordinate = match direction {
            Direction::Right => (old_coordinate.0, old_coordinate.1 + 1),
            Direction::Left => (old_coordinate.0, old_coordinate.1 - 1),
            Direction::Up => (old_coordinate.0 - 1, old_coordinate.1),
            Direction::Down => (old_coordinate.0 + 1, old_coordinate.1),
        };

        print!(
            "Tile: {},{},{}.",
            new_coordinate.0,
            new_coordinate.1,
            self.tiles[new_coordinate.0][new_coordinate.1].get_character()
        );

        print!(
            "Tile: {},{},{}.",
            old_coordinate.0,
            old_coordinate.1,
            self.tiles[old_coordinate.0][old_coordinate.1].get_character()
        );

        match self.tiles[new_coordinate.0][new_coordinate.1] {
            Tile::Food => {
                self.score += 1;
                self.place_food()
            }
            _ => {}
        }

        self.player_coordinate = new_coordinate;
        self.tiles[old_coordinate.0][old_coordinate.1] = Tile::Empty;
        self.tiles[new_coordinate.0][new_coordinate.1] = Tile::Snake;

        self.print();

        writeln!(
            self.stdout,
            "{}Old {:?}, New {:?}, Score {}",
            termion::cursor::Goto(0, 24),
            old_coordinate,
            new_coordinate,
            self.score
        )
        .unwrap();
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
