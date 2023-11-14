extern crate sdl2;

// profiler
pub struct Profiler {
    pub frame_counter: u32,
    pub last_fps_time: u32,
    pub last_memory_print_time: u32,
    pub last_cpu_print_time: u32,
}

#[allow(dead_code)]
impl Profiler {
    pub fn new() -> Self {
        Self {
            frame_counter: 0,
            last_fps_time: unsafe { sdl2::sys::SDL_GetTicks() },
            last_memory_print_time: unsafe { sdl2::sys::SDL_GetTicks() },
            last_cpu_print_time: unsafe { sdl2::sys::SDL_GetTicks() },
        }
    }

    pub fn update(&mut self, current_frame_time: u32) {
        self.frame_counter += 1;

        if current_frame_time - self.last_fps_time >= 1000 {  // Every second
            println!("FPS: {}", self.frame_counter);
            self.frame_counter = 0;
            self.last_fps_time = current_frame_time;
        }

        if current_frame_time - self.last_memory_print_time >= 5000 {  // Every 5 seconds
            // Note: The memory fetching here is a placeholder. In a real-world scenario, you'd use platform-specific methods or tools.
            println!("Memory Usage: {} MB", "PLACEHOLDER");
            self.last_memory_print_time = current_frame_time;
        }

        if current_frame_time - self.last_cpu_print_time >= 10000 {  // Every 10 seconds
            // Note: The CPU fetching here is a placeholder. 
            println!("CPU Usage: {}%", "PLACEHOLDER");
            self.last_cpu_print_time = current_frame_time;
        }
    }
}