extern crate pancurses;
extern crate rand;

use rand::Rng;

mod point;

mod snake;
use snake::Snake;

mod apple;
use apple::Apple;

fn main() {
    let screen = pancurses::initscr();
    let window = pancurses::newwin(20, 20, 0, 0);

    let mut rng = rand::thread_rng();

    let mut snake = Snake::new(4, 4);
    let mut apple = Apple::new(10, 10);

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
            Some(pancurses::Input::KeyLeft) => snake.direction = point::LEFT.clone(),
            Some(pancurses::Input::KeyRight) => snake.direction = point::RIGHT.clone(),
            Some(pancurses::Input::KeyUp) => snake.direction = point::UP.clone(),
            Some(pancurses::Input::KeyDown) => snake.direction = point::DOWN.clone(),
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
