mod two_d;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;

static WORLD_MAP: [[i32; MAP_WIDTH]; MAP_HEIGHT] = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,2,2,2,0,0,0,0,0,0,0,0,2,2,2,2,0,0,0,0,0,0,1],
    [1,0,2,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,1],
    [1,0,2,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,1],
    [1,0,2,2,2,0,0,0,0,0,0,0,0,2,2,2,2,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,2,2,2,2,2,2,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,2,2,2,2,2,2,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

fn cast_ray(player_pos: (f64, f64), player_dir: (f64, f64), camera: &two_d::Camera3D, x: u32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let camera_x = 2.0 * x as f64 / WIDTH as f64 - 1.0; // X-coordinate in camera space
    let ray_dir_x = player_dir.0 + camera.plane.x * camera_x;
    let ray_dir_y = player_dir.1 + camera.plane.y * camera_x;

    // Map position of the current square
    let mut map_x = player_pos.0 as i32;
    let mut map_y = player_pos.1 as i32;

    // Length of ray from one side to next in map
    let delta_dist_x = (1.0 / ray_dir_x).abs();
    let delta_dist_y = (1.0 / ray_dir_y).abs();

    let mut side_dist_x;
    let mut side_dist_y;

    // Calculate step and initial sideDist
    let step_x;
    let step_y;
    let perp_wall_dist;

    if ray_dir_x < 0.0 {
        step_x = -1;
        side_dist_x = (player_pos.0 - map_x as f64) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f64 + 1.0 - player_pos.0) * delta_dist_x;
    }
    if ray_dir_y < 0.0 {
        step_y = -1;
        side_dist_y = (player_pos.1 - map_y as f64) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f64 + 1.0 - player_pos.1) * delta_dist_y;
    }

    // Perform DDA
    let mut hit = 0; // Hit flag
    let mut side = 0; // NS or EW wall
    while hit == 0 {
        // Jump to next map square
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }
        // Check if ray has hit a wall
        if WORLD_MAP[map_x as usize][map_y as usize] > 0 {
            hit = 1;
        }
    }

    // Calculate distance to the point of impact
    if side == 0 {
        perp_wall_dist = (map_x as f64 - player_pos.0 + (1 - step_x) as f64 / 2.0) / ray_dir_x;
    } else {
        perp_wall_dist = (map_y as f64 - player_pos.1 + (1 - step_y) as f64 / 2.0) / ray_dir_y;
    }

    // Calculate height of line to draw on screen
    let line_height = ((HEIGHT as f64 / perp_wall_dist) as i32).max(1);

    // Calculate lowest and highest pixel to fill in current stripe
    let draw_start = (-line_height / 2 + HEIGHT as i32 / 2).max(0);
    let draw_end = (line_height / 2 + HEIGHT as i32 / 2).min(HEIGHT as i32 - 1);

    // Choose wall color
    let wall_color = match WORLD_MAP[map_x as usize][map_y as usize] {
        1 => Color::RGB(255, 0, 0),
        2 => Color::RGB(0, 255, 0),
        3 => Color::RGB(0, 0, 255),
        4 => Color::RGB(255, 255, 0),
        _ => Color::RGB(255, 255, 255),
    };

    // Draw the pixels of the stripe as a vertical line
    canvas.set_draw_color(wall_color);
    canvas.draw_line((x as i32, draw_start), (x as i32, draw_end)).unwrap();
}

fn main() -> Result<(), String> {
    let mut window = two_d::Window::new("My Game", 800, 600)?;
    let mut event_pump = window.sdl_context.event_pump()?;

    let mut player_pos:(f64, f64) = (22.0, 12.0); // Player start position
    let mut player_dir:(f64, f64) = (-1.0, 0.0); // Initial direction vector
    let camera = two_d::Camera3D::new(
        nalgebra::Vector2::new(22.0, 12.0), // Initial position
        nalgebra::Vector2::new(-1.0, 0.0),  // Initial direction
        nalgebra::Vector2::new(0.0, 0.66),  // Initial plane
        nalgebra::Vector2::new(800, 600)    // Viewport size
    );

    let mut last_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f64();
        last_time = now;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        let keys: Vec<Keycode> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        let move_speed:f64 = delta_time * 5.0; // the constant value is in squares/second
        let rot_speed:f64 = delta_time * 3.0; // the constant value is in radians/second

        // Move forward if no wall in front
        if keys.contains(&Keycode::W) {
            if WORLD_MAP[(player_pos.0 + player_dir.0 * move_speed) as usize][player_pos.1 as usize] == 0 {
                player_pos.0 += player_dir.0 * move_speed;
            }
            if WORLD_MAP[player_pos.0 as usize][(player_pos.1 + player_dir.1 * move_speed) as usize] == 0 {
                player_pos.1 += player_dir.1 * move_speed;
            }
        }
        // Move backwards if no wall behind
        if keys.contains(&Keycode::S) {
            if WORLD_MAP[(player_pos.0 - player_dir.0 * move_speed) as usize][player_pos.1 as usize] == 0 {
                player_pos.0 -= player_dir.0 * move_speed;
            }
            if WORLD_MAP[player_pos.0 as usize][(player_pos.1 - player_dir.1 * move_speed) as usize] == 0 {
                player_pos.1 -= player_dir.1 * move_speed;
            }
        }
        // Rotate to the right
        if keys.contains(&Keycode::D) {
            let old_dir_x = player_dir.0;
            player_dir.0 = player_dir.0 * rot_speed.cos() - player_dir.1 * rot_speed.sin();
            player_dir.1 = old_dir_x * rot_speed.sin() + player_dir.1 * rot_speed.cos();
        }
        // Rotate to the left
        if keys.contains(&Keycode::A) {
            let old_dir_x = player_dir.0;
            player_dir.0 = player_dir.0 * (-rot_speed).cos() - player_dir.1 * (-rot_speed).sin();
            player_dir.1 = old_dir_x * (-rot_speed).sin() + player_dir.1 * (-rot_speed).cos();
        }

        // Clear the screen to black (or your floor/ceiling colors)
        window.canvas.set_draw_color(Color::RGB(0, 0, 0));
        window.canvas.clear();

        // Cast rays and draw scene
        for x in 0..WIDTH {
            cast_ray(player_pos, player_dir, &camera, x, &mut window.canvas);
        }

        window.canvas.present();
    }

    Ok(())
}