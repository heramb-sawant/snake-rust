use rand::{thread_rng, Rng};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, clear, AsyncReader};

use std::io::{stdin, stdout, Read, Stdin, Stdout, Write};

/*
    TODO:
        - Print the individual pieces once the board works as intended
        - Snake moves on its own
        - Snake loses if it hits a border
        - Snake increases in size everytime you get food
        - Speed increases
        - Food animation
        - Start and end screen
        - A few tests

        - See if you can get GH tests pipeline working
*/

struct Snake {
    body: Vec<(u16, u16)>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![(1, 1)],
            direction: Direction::Right,
        }
    }

    // fn get_head(&self) -> (usize, usize) {
    //     return self.body[0];
    // }

    // fn increase_length(&mut self, coordinate: (usize, usize)) {
    //     self.body.push(coordinate);
    // }
}

const SNAKE_HEAD: char = '⍥';
const SNAKE_BODY: char = 'o';
const BORDER: char = '|';
const FOOD: char = '*';

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    columns: u16,
    rows: u16,
    snake: Snake,
    // TODO: Should probably also have a reference to food
    score: i16,
    stdout: RawTerminal<Stdout>,
    stdin: AsyncReader,
}

impl Grid {
    fn new(columns: u16, rows: u16, mut stdout: RawTerminal<Stdout>, stdin: AsyncReader) -> Grid {
        write!(stdout, "{}", clear::All).unwrap();

        // Add walls
        let mut i: u16 = 1;
        while i <= rows {
            write!(stdout, "{}{}", termion::cursor::Goto(1, i), BORDER).unwrap();
            write!(stdout, "{}{}", termion::cursor::Goto(rows, i), BORDER).unwrap();
            i += 1
        }

        let mut i: u16 = 1;
        while i <= columns {
            write!(stdout, "{}{}", termion::cursor::Goto(i, 1), BORDER).unwrap();
            write!(stdout, "{}{}", termion::cursor::Goto(i, columns), BORDER).unwrap();
            i += 1
        }

        let snake = Snake::new();
        let snake_head = snake.body[0];
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(snake_head.0 + 1, snake_head.1 + 1),
            SNAKE_HEAD
        )
        .unwrap();

        write!(stdout, "{}", termion::cursor::Goto(0, columns + 1)).unwrap();

        return Grid {
            columns,
            snake: Snake::new(),
            rows,
            score: 0,
            stdout,
            stdin,
        };
    }

    // fn start(&mut self) {
    //     // Set up game
    //     self.place_food();

    //     // TODO: This clears some of the warns I think. Make sure it starts after all the console start output
    //     write!(self.stdout, "{}", clear::All).unwrap();

    //     self.print();

    //     writeln!(
    //         self.stdout,
    //         "{}Arrow keys to move, q to exit.",
    //         termion::cursor::Goto(0, 21),
    //     )
    //     .unwrap();

    //     let mut start_time = SystemTime::now();

    //     loop {
    //         let mut buf = [0];
    //         self.stdin.read(&mut buf).unwrap();

    //         // Clear the current line.
    //         write!(
    //             self.stdout,
    //             "{}{}",
    //             termion::cursor::Goto(0, 22),
    //             termion::clear::CurrentLine
    //         )
    //         .unwrap();

    //         if let Ok(elapsed) = start_time.elapsed() {
    //             if elapsed.as_millis() > 500 {
    //                 println!("{}", elapsed.as_secs());
    //                 self.move_snake();
    //                 start_time = SystemTime::now()
    //             }
    //         }

    //         match buf[0] {
    //             b'q' => break,
    //             b'a' => {
    //                 println!("<left>");
    //                 self.snake.direction = Direction::Left;
    //             }
    //             b'd' => {
    //                 println!("<right>");
    //                 self.snake.direction = Direction::Right;
    //             }
    //             b'w' => {
    //                 println!("<up>");
    //                 self.snake.direction = Direction::Up;
    //             }
    //             b's' => {
    //                 println!("<down>");
    //                 self.snake.direction = Direction::Down;
    //             }
    //             _ => println!("Invalid Move"),
    //         }

    //         self.print();
    //         self.stdout.flush().unwrap();
    //     }

    //     // Show the cursor again before we exit.
    //     write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    // }

    // fn place_food(&mut self) {
    //     let food_column: usize = thread_rng().gen_range(2..self.columns - 2);
    //     let food_row: usize = thread_rng().gen_range(2..self.rows - 2);

    //     self.tiles[food_column][food_row] = Tile::Food;
    // }

    // fn move_snake(&mut self) {
    //     let old_coordinate = self.snake.body[0];

    //     let new_coordinate = match self.snake.direction {
    //         Direction::Right => (old_coordinate.0, old_coordinate.1 + 1),
    //         Direction::Left => (old_coordinate.0, old_coordinate.1 - 1),
    //         Direction::Up => (old_coordinate.0 - 1, old_coordinate.1),
    //         Direction::Down => (old_coordinate.0 + 1, old_coordinate.1),
    //     };

    //     print!(
    //         "Tile: {},{},{}.",
    //         new_coordinate.0,
    //         new_coordinate.1,
    //         self.tiles[new_coordinate.0][new_coordinate.1].get_character()
    //     );

    //     print!(
    //         "Tile: {},{},{}.",
    //         old_coordinate.0,
    //         old_coordinate.1,
    //         self.tiles[old_coordinate.0][old_coordinate.1].get_character()
    //     );

    //     match self.tiles[new_coordinate.0][new_coordinate.1] {
    //         Tile::Food => {
    //             self.score += 1;
    //             self.place_food()
    //         }
    //         _ => {}
    //     }

    //     self.snake.body[0] = new_coordinate;
    //     self.tiles[old_coordinate.0][old_coordinate.1] = Tile::Empty;
    //     self.tiles[new_coordinate.0][new_coordinate.1] = Tile::Snake;

    //     writeln!(
    //         self.stdout,
    //         "{}Old {:?}, New {:?}, Score {}",
    //         termion::cursor::Goto(0, 24),
    //         old_coordinate,
    //         new_coordinate,
    //         self.score
    //     )
    //     .unwrap();
    // }

    // // TODO: get rid of this mut with pointers or whatever you saw in the tutorial.
    // fn print(&mut self) {
    //     // TODO: Update all the tiles to have a coordinate so its easier to reference and print them at a specific position
    //     let mut i = 1;
    //     for columns in self.tiles.iter() {
    //         write!(self.stdout, "{}", termion::cursor::Goto(0, i)).unwrap();
    //         i += 1;
    //         for row in columns.iter() {
    //             write!(self.stdout, "{}", row.get_character()).unwrap()
    //         }
    //         writeln!(self.stdout, "").unwrap()
    //     }
    // }
}

fn main() {
    println!("Starting!");

    let stdin = async_stdin();
    let stdout = stdout().into_raw_mode().unwrap();

    // TODO: Get grid size from input.
    let length = 20;
    let mut grid = Grid::new(length, length, stdout, stdin);

    // grid.start();
}
