extern crate sdl2;

static mut PREVIOUS_IDLE_TIME: u64 = 0;
static mut PREVIOUS_SYSTEM_TIME: u64 = 0;

#[cfg(target_os = "windows")]
mod windows_profiler {
    use std::mem;
    use winapi::shared::minwindef::FILETIME;
    use winapi::um::processthreadsapi::{GetCurrentProcess, GetSystemTimes};
    use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
    
    pub fn get_memory_usage() -> u32 {
        let mut counters: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
        counters.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

        unsafe {
            if GetProcessMemoryInfo(
                GetCurrentProcess(),
                &mut counters,
                mem::size_of_val(&counters) as u32,
            ) != 0
            {
                return (counters.WorkingSetSize / 1024 / 1024) as u32; // Convert from bytes to MB
            }
        }

        0 // In case of failure
    }

    pub fn get_cpu_usage() -> Option<f32> {
        unsafe {
            let mut idle_time = mem::MaybeUninit::<FILETIME>::uninit();
            let mut kernel_time = mem::MaybeUninit::<FILETIME>::uninit();
            let mut user_time = mem::MaybeUninit::<FILETIME>::uninit();

            if GetSystemTimes(
                idle_time.as_mut_ptr(),
                kernel_time.as_mut_ptr(),
                user_time.as_mut_ptr(),
            ) == 0
            {
                return None; // Call failed
            }

            let idle_time = filetime_to_u64(idle_time.assume_init());
            let system_time = filetime_to_u64(kernel_time.assume_init())
                + filetime_to_u64(user_time.assume_init());

            let idle_delta = idle_time - super::PREVIOUS_IDLE_TIME;
            let system_delta = system_time - super::PREVIOUS_SYSTEM_TIME;

            super::PREVIOUS_IDLE_TIME = idle_time;
            super::PREVIOUS_SYSTEM_TIME = system_time;

            if system_delta == 0 {
                return None;
            }

            Some((1.0 - (idle_delta as f32 / system_delta as f32)) * 100.0)
        }
    }

    fn filetime_to_u64(ft: FILETIME) -> u64 {
        ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
    }
}

#[cfg(target_family = "unix")]
mod unix_profiler {
    use std::fs;

    pub fn get_memory_usage() -> u32 {
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

    pub fn get_cpu_usage() -> Option<f32> {
        if let Ok(contents) = fs::read_to_string("/proc/stat") {
            if let Some(line) = contents.lines().next() {
                let values: Vec<&str> = line.split_whitespace().collect();
                if values.len() > 4 {
                    let user_time: f32 = values[1].parse().unwrap_or(0.0);
                    let nice_time: f32 = values[2].parse().unwrap_or(0.0);
                    let system_time: f32 = values[3].parse().unwrap_or(0.0);
                    let idle_time: f32 = values[4].parse().unwrap_or(0.0);
    
                    let total_time = user_time + nice_time + system_time + idle_time;
                    if total_time == 0.0 {
                        return None;
                    }
                    return Some(100.0 * (total_time - idle_time) / total_time);
                }
            }
        }
        None
    }
}

// Re-export platform-specific functions for easy access in the Profiler implementation
#[cfg(target_os = "windows")]
pub use windows_profiler::{get_cpu_usage, get_memory_usage};

#[cfg(target_family = "unix")]
pub use unix_profiler::{get_cpu_usage, get_memory_usage};

// Profiler structure and logic remain the same, using platform-specific functions
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

        if current_frame_time - self.last_fps_time >= 1000 {
            // Every second
            println!("FPS: {}", self.frame_counter);
            self.frame_counter = 0;
            self.last_fps_time = current_frame_time;
        }

        if current_frame_time - self.last_memory_print_time >= 5000 {
            // Every 5 seconds
            let memory_usage = get_memory_usage(); // Fetch real memory usage
            println!("Memory Usage: {} MB", memory_usage);
            self.last_memory_print_time = current_frame_time;
        }

        if current_frame_time - self.last_cpu_print_time >= 10000 {
            // Every 10 seconds
            let cpu_usage = get_cpu_usage().unwrap();
            println!("CPU Usage: {:.2}%", cpu_usage);
            self.last_cpu_print_time = current_frame_time;
        }
    }
}
