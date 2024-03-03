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

    uniform vec2 uImagePos; // Uniform for image position
    uniform float uScale; // Uniform for image scale
    uniform int uImageIndex; // Uniform to differentiate images

    out vec2 TexCoord;

    void main() {
        vec3 pos = vec3(aPos.x * uScale, aPos.y * uScale, aPos.z);
        // Adjust position based on image index to avoid overlap
        if(uImageIndex == 1) {
            pos.x += 0.1; // Slightly move the second image to the right
        }
        gl_Position = vec4(pos.x + uImagePos.x, pos.y + uImagePos.y, pos.z, 1.0);
        TexCoord = aTexCoord;
    }
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;

    in vec2 TexCoord;

    // Single texture sampler
    uniform sampler2D textureSampler;

    void main() {
        FragColor = texture(textureSampler, TexCoord);
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
    pub pos_x: f32,
    pub pos_y: f32,
    pub scale: f32,
    pub is_dragging: bool,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Image {
    pub fn new(texture_id: u32, width: u32, height: u32, pos_x: f32, pos_y: f32) -> Self {
        Image {
            texture_id,
            width,
            height,
            pos_x,
            pos_y,
            scale: 1.0,
            is_dragging: false,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}