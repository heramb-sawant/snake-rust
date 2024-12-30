use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
enum Tile {
    Empty,
    Player,
    Enemy,
    Wall,
    Start,
    End,
}

impl Tile {
    fn get_character(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Player => 'P',
            Tile::Enemy => 'E',
            Tile::Wall => '+',
            Tile::Start => '~',
            Tile::End => '=',
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
        let empty_grid = vec![vec![Tile::Empty; rows]; columns];

        return Grid {
            tiles: empty_grid,
            columns,
            rows,
        };
    }

    fn print(&self) {
        for columns in self.tiles.iter() {
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
