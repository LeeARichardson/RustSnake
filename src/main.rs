extern crate pancurses;
extern crate rand;

use std::collections::LinkedList;
use rand::Rng;

struct Point {
    x: i32,
    y: i32,
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    direction: Point,
    last_removed_body_position: Option<Point>
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        Snake {
            head: Point::new(x, y),
            body: LinkedList::new(),
            direction: Point::new(1, 0), // Right
            last_removed_body_position: None
        }
    }

    pub fn movement(&mut self) {
        self.body.push_front(self.head.clone());
        self.head = Point::new(self.head.x + self.direction.x,
                               self.head.y + self.direction.y);

        self.last_removed_body_position = self.body.pop_back();
    }

    pub fn grow(&mut self) {
        self.body.push_back(self.last_removed_body_position.clone().unwrap());
    }
}

struct Apple {
    location: Point,
}

impl Apple {
    pub fn new(x: i32, y: i32) -> Apple {
        Apple { location: Point { x: x, y: y } }
    }
}

fn main() {
    let screen = pancurses::initscr();
    let window = pancurses::newwin(20, 20, 0, 0);

    let mut rng = rand::thread_rng();

    let mut snake = Snake::new(4, 4);
    let mut apple = Apple::new(10, 10);

    let left = Point::new(-1, 0);
    let right = Point::new(1, 0);
    let up = Point::new(0, -1);
    let down = Point::new(0, 1);


    pancurses::noecho();
    pancurses::curs_set(0);
    pancurses::half_delay(5);
    screen.keypad(true);

    snake.movement();
    snake.grow();
    snake.movement();
    snake.grow();

    loop {
        match screen.getch() {
            Some(pancurses::Input::Character('q')) => break,
            Some(pancurses::Input::KeyLeft) => snake.direction = left.clone(),
            Some(pancurses::Input::KeyRight) => snake.direction = right.clone(),
            Some(pancurses::Input::KeyUp) => snake.direction = up.clone(),
            Some(pancurses::Input::KeyDown) => snake.direction = down.clone(),
            Some(input) => {}
            None => (),
        }

        let dead = snake.body.iter().any(|ref body_part| snake.head == **body_part);

        if dead {
            break;
        }

        snake.movement();

        if snake.head == apple.location {
            let x: i32 = rng.gen_range(0, window.get_max_x());
            let y: i32 = rng.gen_range(0, window.get_max_y());

            apple = Apple::new(x, y);
            snake.grow()
        }

        render(&window, &snake, &apple);
    }

    pancurses::nocbreak();
    window.getch();

    pancurses::endwin();
}

fn render(window: &pancurses::Window, snake: &Snake, apple: &Apple) {
    window.clear();

    window.mvprintw(apple.location.y, apple.location.x, "0");

    for segment in &snake.body {
        window.mvprintw(segment.y, segment.x, "#");
    }

    window.mvprintw(snake.head.y, snake.head.x, "s");

    window.refresh();
}
