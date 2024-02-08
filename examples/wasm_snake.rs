extern crate sdl2;
mod emscripten;
mod two_d;
use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::{Instant, Duration};

use rand::Rng;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const SNAKE_SIZE: u32 = 20;

pub struct Snake {
    pub body: Vec<two_d::Rect>,
    pub direction: Direction,
}

#[derive(PartialEq)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new() -> Self {
        let initial_position = two_d::Rect::new((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32, SNAKE_SIZE, SNAKE_SIZE);
        Snake {
            body: vec![initial_position],
            direction: Direction::None,
        }
    }

    pub fn update(&mut self) {
        let mut new_head = *self.body.first().expect("Snake has no body!");
        match self.direction {
            Direction::Up => new_head.set_y(new_head.y() - SNAKE_SIZE as i32),
            Direction::Down => new_head.set_y(new_head.y() + SNAKE_SIZE as i32),
            Direction::Left => new_head.set_x(new_head.x() - SNAKE_SIZE as i32),
            Direction::Right => new_head.set_x(new_head.x() + SNAKE_SIZE as i32),
            Direction::None => {}
        }

        self.body.insert(0, new_head); // Add new head to the front of the snake
        self.body.pop(); // Remove the last segment of the snake
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        for segment in &self.body {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(segment.to_sdl()).expect("Could not draw snake segment.");
        }
    }
}

fn spawn_food() -> two_d::Rect {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..WINDOW_WIDTH / SNAKE_SIZE) * SNAKE_SIZE;
    let y = rng.gen_range(0..WINDOW_HEIGHT / SNAKE_SIZE) * SNAKE_SIZE;
    two_d::Rect::new(x as i32, y as i32, SNAKE_SIZE, SNAKE_SIZE)
}

fn main_loop(ctx: Rc<RefCell<sdl2::Sdl>>, snake: Rc<RefCell<Snake>>, canvas: Rc<RefCell<WindowCanvas>>, food: Rc<RefCell<two_d::Rect>>) -> impl FnMut() {
    let mut input_handler = two_d::InputHandler::new(&ctx.borrow()).unwrap();
    let update_interval = Duration::from_millis(150);
    let mut last_update = Instant::now();

    move || {
        // Assume `input_handler` is an instance of `InputHandler` available in this closure's scope
        let events = input_handler.poll_events(); // Poll and process events, updating `InputHandler`'s state

        // Now instead of polling events directly, check the `InputHandler`'s state
        if input_handler.is_key_pressed(Keycode::Escape) {
            process::exit(1);
        }

        if input_handler.is_key_pressed(Keycode::Left) && snake.borrow().direction != Direction::Right {
            snake.borrow_mut().direction = Direction::Left;
        }
        if input_handler.is_key_pressed(Keycode::Right) && snake.borrow().direction != Direction::Left {
            snake.borrow_mut().direction = Direction::Right;
        }
        if input_handler.is_key_pressed(Keycode::Up) && snake.borrow().direction != Direction::Down {
            snake.borrow_mut().direction = Direction::Up;
        }
        if input_handler.is_key_pressed(Keycode::Down) && snake.borrow().direction != Direction::Up {
            snake.borrow_mut().direction = Direction::Down;
        }

        // Handle other events if needed, such as checking for Quit event
        for event in events {
            if let Event::Quit { .. } = event {
                process::exit(1);
            }
        }

        if last_update.elapsed() >= update_interval {
            let mut snake_borrowed = snake.borrow_mut();
            snake_borrowed.update();
        
            // Check if the snake has eaten the food
            let head_position = *snake_borrowed.body.first().expect("Snake has no body");
            let food_position = *food.borrow();
        
            if head_position == food_position {
                // Temporarily store the last segment
                let last_segment = snake_borrowed.body.last().expect("Snake has no body").clone();
                // Grow the snake by pushing the stored segment
                snake_borrowed.body.push(last_segment.into());
                // Respawn food
                *food.borrow_mut() = spawn_food();
            }
        
            last_update = Instant::now();
        }

        // Drawing the snake and the food
        canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
        canvas.borrow_mut().clear();
        snake.borrow().draw(&mut canvas.borrow_mut());

        // Draw the food
        canvas.borrow_mut().set_draw_color(Color::RGB(255, 0, 0)); // Food color
        canvas.borrow_mut().fill_rect(food.borrow().to_sdl()).expect("Could not draw the food");

        canvas.borrow_mut().present();
    }
}

fn main() {
    let window = two_d::Window::new("Snake Game", WINDOW_WIDTH, WINDOW_HEIGHT, false).unwrap();

    let snake = Snake::new(); // Create the initial snake state

    let sdl_context = Rc::new(RefCell::new(window.sdl_context));
    let snake = Rc::new(RefCell::new(snake));
    let canvas = Rc::new(RefCell::new(window.canvas));

    let food = Rc::new(RefCell::new(spawn_food()));

    // Main loop logic for different platforms
    #[cfg(target_family = "wasm")]
    emscripten::emscripten::set_main_loop_callback(main_loop(Rc::clone(&sdl_context), Rc::clone(&snake), Rc::clone(&canvas), Rc::clone(&food)));

    #[cfg(not(target_family = "wasm"))]
    {
        use std::thread::sleep;
        let mut loop_closure = main_loop(Rc::clone(&sdl_context), Rc::clone(&snake), Rc::clone(&canvas), Rc::clone(&food));
        loop {
            loop_closure();
            sleep(Duration::from_millis(10)); // Control loop speed
        }
    }
}
