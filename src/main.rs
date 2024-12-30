use std::fmt::{Display, Formatter, Result};

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

    let grid = Grid::new(20, 20);

    grid.print();
}
