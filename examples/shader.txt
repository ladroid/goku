// fn compile_shader(src: &str, ty: GLenum) -> Result<GLuint, String> {
//     let shader;
//     unsafe {
//         shader = gl::CreateShader(ty);
//         let c_str = std::ffi::CString::new(src.as_bytes()).unwrap();
//         gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
//         gl::CompileShader(shader);

//         let mut status = gl::FALSE as GLint;
//         gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

//         if status != (gl::TRUE as GLint) {
//             let mut len = 0;
//             gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
//             let mut buf = Vec::with_capacity(len as usize);
//             buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
//             gl::GetShaderInfoLog(
//                 shader,
//                 len,
//                 std::ptr::null_mut(),
//                 buf.as_mut_ptr() as *mut GLchar,
//             );
//             return Err(std::str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8").to_owned());
//         }
//     }
//     Ok(shader)
// }

// fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> Result<GLuint, String> {
//     let program;
//     unsafe {
//         program = gl::CreateProgram();
//         gl::AttachShader(program, vertex_shader);
//         gl::AttachShader(program, fragment_shader);
//         gl::LinkProgram(program);

//         let mut status = gl::FALSE as GLint;
//         gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

//         if status != (gl::TRUE as GLint) {
//             let mut len = 0;
//             gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
//             let mut buf = Vec::with_capacity(len as usize);
//             buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
//             gl::GetProgramInfoLog(
//                 program,
//                 len,
//                 std::ptr::null_mut(),
//                 buf.as_mut_ptr() as *mut GLchar,
//             );
//             return Err(std::str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8").to_owned());
//         }
//     }
//     Ok(program)
// }