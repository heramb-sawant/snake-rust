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

    fn shift(&mut self) {
        let snake_head = self.body[0];

        let new_coordinate = match self.direction {
            Direction::Right => (snake_head.0 + 1, snake_head.1),
            Direction::Left => (snake_head.0 - 1, snake_head.1),
            Direction::Up => (snake_head.0, snake_head.1 - 1),
            Direction::Down => (snake_head.0, snake_head.1 + 1),
        };

        let i = 1;
        while i < self.body.len() {
            self.body[i] = self.body[i - 1]
        }

        self.body[0] = new_coordinate;
    }

    fn increase_length(&mut self, coordinate: (u16, u16)) {
        self.body.push(coordinate);
    }
}

const SNAKE_HEAD: char = '⍥';
const EMPTY: char = ' ';
const SNAKE_BODY: char = 'o';
const BORDER: char = '|';
const FOOD: char = '*';

#[derive(Debug)]
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
    // TODO: Add speed
}

impl Grid {
    fn new(columns: u16, rows: u16, mut stdout: RawTerminal<Stdout>, stdin: AsyncReader) -> Grid {
        write!(stdout, "{}", clear::All).unwrap();

        // Add walls
        // TODO: make sure height and with are correct, seems like the termion print only goes to 19
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
                if elapsed.as_millis() > 500 {
                    self.move_snake();
                    start_time = SystemTime::now()
                }
            }

            match buf[0] {
                b'q' => break,
                b'a' => {
                    self.snake.direction = Direction::Left;
                }
                b'd' => {
                    self.snake.direction = Direction::Right;
                }
                b'w' => {
                    self.snake.direction = Direction::Up;
                }
                b's' => {
                    self.snake.direction = Direction::Down;
                }
                _ => {}
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

    fn place_food(&mut self) {
        let food_column = thread_rng().gen_range(2..self.columns - 1);
        let food_row = thread_rng().gen_range(2..self.rows - 1);

        self.food = (food_column, food_row);
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(food_row + 1, food_column + 1),
            FOOD
        )
        .unwrap();
    }

    fn move_snake(&mut self) {
        let old_coordinate = self.snake.body[0];
        let old_snake_end = self.snake.body[self.snake.body.len() - 1]; // TODO: Try to use the .first/.end methods instead

        self.snake.shift();

        let i = 0;
        while i < self.snake.body.len() {
            let snake_bit = self.snake.body[i];
            let snake_part = if i == 0 { SNAKE_HEAD } else { SNAKE_BODY };
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(snake_bit.0 + 1, snake_bit.1 + 1),
                snake_part
            )
            .unwrap();
        }

        let snake_head = self.snake.body[0];
        if snake_head == self.food {
            self.score += 1;
            self.place_food();
            self.snake.increase_length(old_snake_end);
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(old_snake_end.0 + 1, old_snake_end.1 + 1),
                SNAKE_BODY
            )
            .unwrap();
        } else {
            write!(
                self.stdout,
                "{}{}",
                termion::cursor::Goto(old_snake_end.0 + 1, old_snake_end.1 + 1),
                EMPTY
            )
            .unwrap();
        }

        write!(
            self.stdout,
            "{}Food: {:?}, Snake: {:?}, Direction: {:?}, Old {:?}, Score {}",
            termion::cursor::Goto(0, self.columns + 2),
            self.food,
            self.snake.body[0],
            self.snake.direction,
            old_coordinate,
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
