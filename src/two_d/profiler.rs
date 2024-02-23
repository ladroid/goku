extern crate sdl2;

#[cfg(target_os = "windows")]
fn get_memory_usage() -> u32 {
    use winapi::um::psapi::PROCESS_MEMORY_COUNTERS;
    use winapi::um::psapi::GetProcessMemoryInfo;
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use std::mem;

    let mut counters: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
    counters.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

    unsafe {
        if GetProcessMemoryInfo(GetCurrentProcess(), &mut counters, mem::size_of_val(&counters) as u32) != 0 {
            return (counters.WorkingSetSize / 1024 / 1024) as u32; // Convert from bytes to MB
        }
    }

    0 // In case of failure
}

#[cfg(target_os = "unix")]
fn get_memory_usage() -> u32 {
    use std::fs;

    if let Ok(statm) = fs::read_to_string("/proc/self/statm") {
        if let Some(mem) = statm.split_whitespace().next() {
            if let Ok(pages) = mem.parse::<u64>() {
                let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u64;
                return (pages * page_size / 1024 / 1024) as u32; // Convert from bytes to MB
            }
        }
    }

    0 // In case of failure or inability to read
}


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
            let memory_usage = get_memory_usage(); // Fetch real memory usage
            println!("Memory Usage: {} MB", memory_usage);
            self.last_memory_print_time = current_frame_time;
        }

        if current_frame_time - self.last_cpu_print_time >= 10000 {  // Every 10 seconds
            // TODO: Note: The CPU fetching here is a placeholder. 
            println!("CPU Usage: {}%", "PLACEHOLDER");
            self.last_cpu_print_time = current_frame_time;
        }
    }
}