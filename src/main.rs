extern crate pancurses;
extern crate rand;

use rand::Rng;

mod point;

mod snake;
use snake::Snake;

mod apple;
use apple::Apple;

fn setup_curses() {
    pancurses::noecho();
    pancurses::curs_set(0);
    pancurses::half_delay(5);
    pancurses::start_color();

    pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    pancurses::init_pair(0, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
}

fn main() {
    const GAME_FIELD_WIDTH: i32 = 20;
    const GAME_FIELD_HEIGHT: i32 = 20;

    let screen = pancurses::initscr();
    let window = pancurses::newwin(GAME_FIELD_WIDTH + 1, GAME_FIELD_HEIGHT + 1, 0, 0);

    let mut rng = rand::thread_rng();

    let mut snake = Snake::new(4, 4);
    let mut apple = Apple::new(10, 10);

    screen.keypad(true);

    setup_curses();

    snake.movement();
    snake.grow();
    snake.movement();
    snake.grow();

    loop {
        match screen.getch() {
            Some(pancurses::Input::Character('q')) => break,
            Some(pancurses::Input::KeyLeft) => snake.direction = point::LEFT.clone(),
            Some(pancurses::Input::KeyRight) => snake.direction = point::RIGHT.clone(),
            Some(pancurses::Input::KeyUp) => snake.direction = point::UP.clone(),
            Some(pancurses::Input::KeyDown) => snake.direction = point::DOWN.clone(),
            Some(input) => {}
            None => (),
        }

        let has_hit_self = snake.body.iter().any(|ref body_part| snake.head == **body_part);
        let has_hit_wall = snake.head.x <= 0 || snake.head.x >= GAME_FIELD_WIDTH ||
                           snake.head.y <= 0 ||
                           snake.head.y >= GAME_FIELD_HEIGHT;

        let dead = has_hit_self || has_hit_wall;

        if dead {
            break;
        }

        snake.movement();

        if snake.head == apple.location {
            let x: i32 = rng.gen_range(0, GAME_FIELD_WIDTH);
            let y: i32 = rng.gen_range(0, GAME_FIELD_HEIGHT);

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

    window.draw_box('|', '-');

    window.color_set(1);
    window.mvprintw(apple.location.y, apple.location.x, "0");
    window.color_set(0);

    for segment in &snake.body {
        window.mvprintw(segment.y, segment.x, "#");
    }

    window.mvprintw(snake.head.y, snake.head.x, "s");


    window.refresh();
}
