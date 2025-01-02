use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter, Result};

/*
    TODO:
        - Randomly generate monsters on the board
        - Get display using termion or crossterm
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
}

impl Grid {
    fn new(columns: usize, rows: usize) -> Grid {
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

        let enemy_column: usize = thread_rng().gen_range(2..19);
        let enemy_row: usize = thread_rng().gen_range(2..19);

        empty_grid[enemy_column][enemy_row] = Tile::Enemy;

        return Grid {
            tiles: empty_grid,
            columns,
            rows,
        };
    }

    fn print(&self) {
        for columns in self.tiles.iter().rev() {
            for row in columns.iter() {
                print!("{}", row.get_character())
            }
            println!("")
        }
    }
}

fn main() {
    println!("Starting!");

    let length = 20;
    let grid = Grid::new(length, length);

    grid.print();
}
