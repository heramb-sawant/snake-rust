use rand::{thread_rng, Rng};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor, style};

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

struct Grid {
    tiles: Vec<Vec<Tile>>,
    columns: usize,
    rows: usize,
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

        empty_grid[1][1] = Tile::Player;
        empty_grid[columns - 2][rows - 2] = Tile::Exit;

        let enemy_column: usize = thread_rng().gen_range(2..18);
        let enemy_row: usize = thread_rng().gen_range(2..18);

        empty_grid[enemy_column][enemy_row] = Tile::Enemy;

        return Grid {
            tiles: empty_grid,
            columns,
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

        write!(
            self.stdout,
            "{}Arrow keys to move, q to exit.{}",
            termion::cursor::Goto(0, 21),
            termion::cursor::Hide
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
                Key::Left => println!("<left>"),
                Key::Right => println!("<right>"),
                Key::Up => println!("<up>"),
                Key::Down => println!("<down>"),
                _ => println!("Invalid Move"),
            }

            // Flush again.
            self.stdout.flush().unwrap();
        }

        // Show the cursor again before we exit.
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }

    // TODO: get rid of this mut with pointers or whatever you saw in the tutorial.
    fn print(&mut self) {
        // TODO: Update all the tiles to have a coordinate so its easier to reference and print them at a specific position
        let mut i = 1;
        for columns in self.tiles.iter().rev() {
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
