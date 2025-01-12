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
const EMPTY: char = ' ';
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
    food: (u16, u16),
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

        // Snake
        let snake = Snake::new();
        let snake_head = snake.body[0];
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(snake_head.0 + 1, snake_head.1 + 1),
            SNAKE_HEAD
        )
        .unwrap();

        // Food
        let food = (rows / 2, columns / 2);
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(food.0 + 1, food.1 + 1),
            FOOD
        )
        .unwrap();

        return Grid {
            columns,
            snake: Snake::new(),
            food,
            rows,
            score: 0,
            stdout,
            stdin,
        };
    }

    fn start(&mut self) {
        writeln!(
            self.stdout,
            "{}Arrow keys to move, q to exit.",
            termion::cursor::Goto(0, self.columns + 3),
        )
        .unwrap();

        let mut start_time = SystemTime::now();

        loop {
            let mut buf = [0];
            self.stdin.read(&mut buf).unwrap();

            // Clear the current line.
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(0, self.columns + 1),
                termion::clear::CurrentLine
            )
            .unwrap();

            if let Ok(elapsed) = start_time.elapsed() {
                if elapsed.as_millis() > 1000 {
                    self.move_snake();
                    start_time = SystemTime::now()
                }
            }

            match buf[0] {
                b'q' => break,
                b'a' => {
                    print!("<left>");
                    self.snake.direction = Direction::Left;
                }
                b'd' => {
                    print!("<right>");
                    self.snake.direction = Direction::Right;
                }
                b'w' => {
                    print!("<up>");
                    self.snake.direction = Direction::Up;
                }
                b's' => {
                    print!("<down>");
                    self.snake.direction = Direction::Down;
                }
                _ => print!("Invalid Move"),
            }

            self.stdout.flush().unwrap();
        }

        // Show the cursor again before we exit.
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(0, self.columns + 4),
            termion::cursor::Show
        )
        .unwrap();
    }

    // fn place_food(&mut self) {
    //     let food_column = thread_rng().gen_range(2..self.columns - 2);
    //     let food_row = thread_rng().gen_range(2..self.rows - 2);

    //     self.food = (food_column, food_row);
    //     write!(
    //         self.stdout,
    //         "{}{}",
    //         termion::cursor::Goto(food_row, food_column),
    //         FOOD
    //     )
    //     .unwrap();
    // }

    fn move_snake(&mut self) {
        let old_coordinate = self.snake.body[0];

        let new_coordinate = match self.snake.direction {
            Direction::Right => (old_coordinate.0 + 1, old_coordinate.1),
            Direction::Left => (old_coordinate.0 - 1, old_coordinate.1),
            Direction::Up => (old_coordinate.0, old_coordinate.1 - 1),
            Direction::Down => (old_coordinate.0, old_coordinate.1 + 1),
        };

        self.snake.body[0] = new_coordinate;
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(new_coordinate.0 + 1, new_coordinate.1 + 1),
            SNAKE_HEAD
        )
        .unwrap();
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(old_coordinate.0 + 1, old_coordinate.1 + 1),
            EMPTY
        )
        .unwrap();

        writeln!(
            self.stdout,
            "{}Food: {:?}, Snake: {:?}, Old {:?}, New {:?}, Score {}",
            termion::cursor::Goto(0, self.columns + 2),
            self.food,
            self.snake.body[0],
            old_coordinate,
            new_coordinate,
            self.score
        )
        .unwrap();
    }
}

fn main() {
    println!("Starting!");

    let stdin = async_stdin();
    let stdout = stdout().into_raw_mode().unwrap();

    let length = 20;
    let mut grid = Grid::new(length, length, stdout, stdin);

    grid.start();
}
