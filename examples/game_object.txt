// pub fn update_position(&mut self, event: sdl2::event::Event, colliders: &Vec<Rect>) {
    //     let mut new_x = self.x;
    //     let mut new_y = self.y;

    //     match event {
    //         sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
    //             match keycode {
    //                 sdl2::keyboard::Keycode::Left => { new_x -= 10; },
    //                 sdl2::keyboard::Keycode::Right => { new_x += 10; },
    //                 sdl2::keyboard::Keycode::Up => { new_y -= 10; },
    //                 sdl2::keyboard::Keycode::Down => { new_y += 10; },
    //                 _ => {},
    //             }
    //         },
    //         _ => {},
    //     }

    //     let new_collider = sdl2::rect::Rect::new(new_x, new_y, self.collider.width(), self.collider.height());

    //     // Check for collisions with other colliders
    //     let mut collision = false;
    //     for collider in colliders.iter() {
    //         if new_collider.has_intersection(*collider) {
    //             collision = true;
    //             break;
    //         }
    //     }

    //     if !collision {
    //         self.x = new_x;
    //         self.y = new_y;
    //         self.collider = new_collider;
    //     }
    // }