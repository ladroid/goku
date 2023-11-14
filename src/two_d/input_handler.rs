use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::EventPump;
use sdl2::controller::GameController;

// Input handler
pub struct InputHandler {
    event_pump: EventPump,
    controller: Option<GameController>,
    keys_pressed: HashSet<Keycode>,
    mouse_button_pressed: bool,
    mouse_position: (i32, i32),
}

#[allow(dead_code)]
impl InputHandler {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, Box<dyn std::error::Error>> {
        let event_pump = sdl_context.event_pump()?;
        
        let mut controller = None;
        let game_controller_subsystem = sdl_context.game_controller()?;
        for i in 0..game_controller_subsystem.num_joysticks()? {
            if game_controller_subsystem.is_game_controller(i) {
                controller = game_controller_subsystem.open(i).ok();
                break;
            }
        }
        
        Ok(Self {
            event_pump,
            controller,
            keys_pressed: HashSet::new(),
            mouse_button_pressed: false,
            mouse_position: (0, 0),
        })
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        self.mouse_button_pressed = false;
        
        let mut events = Vec::new();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.keys_pressed.insert(keycode);
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    self.keys_pressed.remove(&keycode);
                },
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        self.mouse_position = (x, y);
                        self.mouse_button_pressed = true;
                    }
                },
                _ => {}
            }
            events.push(event);
        }

        events
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.keys_pressed.contains(&keycode)
    }

    pub fn is_button_pressed(&self, button: sdl2::controller::Button) -> bool {
        if let Some(controller) = &self.controller {
            controller.button(button)
        } else {
            false
        }
    }

    pub fn is_mouse_button_pressed(&self) -> bool {
        self.mouse_button_pressed
    }

    pub fn get_mouse_position(&self) -> (i32, i32) {
        self.mouse_position
    }

    pub fn get_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }
}