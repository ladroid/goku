use sdl2::image::LoadSurface;
use std::ffi::CString;
use std::ptr;
use std::str;

pub fn sdl_surface_to_gl_texture(surface: &sdl2::surface::Surface) -> Result<u32, String> {
    let mut texture_id: u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        let format = match surface.pixel_format_enum() {
            sdl2::pixels::PixelFormatEnum::RGB24 => gl::RGB,
            sdl2::pixels::PixelFormatEnum::RGBA32 => gl::RGBA,
            // Ensure correct format is specified for loaded image
            sdl2::pixels::PixelFormatEnum::RGB888 => gl::RGB,
            sdl2::pixels::PixelFormatEnum::ARGB8888 => gl::RGBA,
            _ => return Err("Unsupported pixel format".to_string()),
        };

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            surface.width() as i32,
            surface.height() as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            surface.without_lock().unwrap().as_ptr() as *const _,
        );
    }

    Ok(texture_id)
}

pub fn compile_shader(shader_type: gl::types::GLenum, source: &str) -> Result<u32, String> {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = CString::new(source.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut gl::types::GLchar);
            return Err(str::from_utf8(&buf).unwrap().to_owned());
        }
    }
    Ok(shader)
}

pub fn link_program(vert_shader: u32, frag_shader: u32) -> Result<u32, String> {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vert_shader);
        gl::AttachShader(program, frag_shader);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            let mut len: gl::types::GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut gl::types::GLchar);
            return Err(str::from_utf8(&buf).unwrap().to_owned());
        }
    }
    Ok(program)
}

pub const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexCoord;

    uniform vec2 uImagePos;
    uniform float uScale;
    uniform int uCurrentFrame;
    uniform int uFrames;
    uniform int uCurrentRow;
    uniform int uRows;

    out vec2 TexCoord;
    out float Frame;
    out float TotalFrames;
    out float CurrentRow;
    out float TotalRows;

    void main() {
        vec3 pos = vec3(aPos.x * uScale, aPos.y * uScale, aPos.z);
        gl_Position = vec4(pos.x + uImagePos.x, pos.y + uImagePos.y, pos.z, 1.0);
        TexCoord = aTexCoord;
        Frame = float(uCurrentFrame);
        TotalFrames = float(uFrames);
        CurrentRow = float(uCurrentRow);
        TotalRows = float(uRows);
    }
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;

    in vec2 TexCoord;
    in float Frame;
    in float TotalFrames;
    in float CurrentRow;
    in float TotalRows;

    uniform sampler2D textureSampler;

    void main() {
        // Calculate the width of a single frame and the height of a single row
        float frameWidth = 1.0 / TotalFrames;
        float rowHeight = 1.0 / TotalRows;
        // Calculate the offset based on the current frame and row
        float offsetX = frameWidth * Frame;
        float offsetY = rowHeight * CurrentRow;
        // Adjust TexCoord based on the current frame and row
        vec2 frameTexCoord = vec2((TexCoord.x * frameWidth) + offsetX, (TexCoord.y * rowHeight) + offsetY);
        FragColor = texture(textureSampler, frameTexCoord);
    }
"#;

// Add this struct to manage the viewport state
pub struct ViewportState {
    pub offset_x: f32,
    pub offset_y: f32,
}

impl ViewportState {
    pub fn new() -> Self {
        ViewportState {
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
    
    // Function to update the viewport offset based on keyboard input
    pub fn update_offset(&mut self, keycode: sdl2::keyboard::Keycode, delta: f32) {
        match keycode {
            sdl2::keyboard::Keycode::W => self.offset_y -= delta,
            sdl2::keyboard::Keycode::S => self.offset_y += delta,
            sdl2::keyboard::Keycode::A => self.offset_x -= delta,
            sdl2::keyboard::Keycode::D => self.offset_x += delta,
            _ => {},
        }
    }
}

// Define a struct to hold information about each image
pub struct Image {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub frames: usize,
    pub rows: usize,
    pub current_frame: usize,
    pub current_row: usize,
    pub selected_row: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub scale: f32,
    pub is_dragging: bool,
    pub offset_x: f32,
    pub offset_y: f32,
    pub animation: bool,
    pub last_update: std::time::Instant,
    pub frame_duration: std::time::Duration,
}

impl Image {
    pub fn new(texture_id: u32, width: u32, height: u32, frames: usize, rows: usize, pos_x: f32, pos_y: f32) -> Self {
        Image {
            texture_id,
            width,
            height,
            frames,
            rows,
            current_frame: 0,
            current_row: 0,
            selected_row: 0,
            pos_x,
            pos_y,
            scale: 1.0,
            is_dragging: false,
            offset_x: 0.0,
            offset_y: 0.0,
            animation: false,
            last_update: std::time::Instant::now(),
            frame_duration: std::time::Duration::from_millis(100),
        }
    }

    pub fn update(&mut self) {
        if self.animation && self.last_update.elapsed() >= self.frame_duration {
            self.current_frame = (self.current_frame + 1) % self.frames;
            if self.current_frame == 0 {
                self.current_row = (self.current_row + 1) % self.rows;
            }
            self.last_update = std::time::Instant::now();
        }
    }

    pub fn set_selected_row(&mut self, row: usize) {
        self.selected_row = row;
        self.current_row = row; // Ensure the current row updates to the selected row
    }
}

pub fn load_texture_from_drop_event(event: &sdl2::event::Event) -> Result<(u32, u32, u32), String> {
    if let sdl2::event::Event::DropFile { filename, .. } = event {
        let surface = sdl2::surface::Surface::from_file(std::path::Path::new(&filename))?;
        let texture_id = sdl_surface_to_gl_texture(&surface)?;
        Ok((texture_id, surface.width(), surface.height()))
    } else {
        Err("No file dropped".to_string())
    }
}