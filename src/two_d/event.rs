extern crate sdl2;

use sdl2::keyboard::Keycode;

#[allow(dead_code)]
pub enum KeyEvent {
    Left,
    Right,
    Up,
    Down,
    Escape,
    Other(Keycode),
}

// #[allow(dead_code)]
// pub struct MouseButtonEvent {
//     pub button: sdl2::mouse::MouseButton,
//     pub x: i32,
//     pub y: i32,
// }

// #[allow(dead_code)]
// pub enum MouseButton {
//     Left,
//     Right,
//     Middle,
//     // ... other buttons as necessary ...
// }

#[allow(dead_code)]
pub enum GEvent {
    Quit,
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    // MouseButtonDown(MouseButtonEvent),
}

#[allow(dead_code)]
pub fn from_sdl_event(event: sdl2::event::Event) -> Option<GEvent> {
    match event {
        sdl2::event::Event::Quit { .. } => Some(GEvent::Quit),
        sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
            let key = match keycode {
                Keycode::Left => KeyEvent::Left,
                Keycode::Right => KeyEvent::Right,
                Keycode::Up => KeyEvent::Up,
                Keycode::Down => KeyEvent::Down,
                Keycode::Escape => KeyEvent::Escape,
                other => KeyEvent::Other(other),
            };
            Some(GEvent::KeyDown(key))
        }
        sdl2::event::Event::KeyUp { keycode: Some(keycode), .. } => {
            let key = match keycode {
                Keycode::Left => KeyEvent::Left,
                Keycode::Right => KeyEvent::Right,
                Keycode::Up => KeyEvent::Up,
                Keycode::Down => KeyEvent::Down,
                Keycode::Escape => KeyEvent::Escape,
                other => KeyEvent::Other(other),
            };
            Some(GEvent::KeyUp(key))
        }
        // sdl2::event::Event::MouseButtonDown { x, y, mouse_btn, .. } => match mouse_btn {
        //     sdl2::mouse::MouseButton::Left => Some(GEvent::MouseButtonDown(MouseButtonEvent { button: sdl2::mouse::MouseButton::Left, x, y })),
        //     sdl2::mouse::MouseButton::Right => Some(GEvent::MouseButtonDown(MouseButtonEvent { button: sdl2::mouse::MouseButton::Right, x, y })),
        //     sdl2::mouse::MouseButton::Middle => Some(GEvent::MouseButtonDown(MouseButtonEvent { button: sdl2::mouse::MouseButton::Middle, x, y })),
        //     // ... other mouse button mappings ...
        //     _ => None,
        // },
        _ => None,
    }
}