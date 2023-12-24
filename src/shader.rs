use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::fs;

pub const VS_SRC: &'static str = "resources/shaders/vertexShader.glsl";
pub const FS_SRC: &'static str = "resources/shaders/fragmentShader.glsl";
pub const TCS_SRC: &'static str = "resources/shaders/tessellationControlShader.glsl";
pub const TES_SRC: &'static str = "resources/shaders/tessellationEvaluationShader.glsl";
pub const FSM_SRC: &'static str = "resources/shaders/fragmentMeshShader.glsl";


pub struct Shader {
    pub id : GLuint,
}

impl Shader {
    pub fn new(file_path: &str, shader_type: GLenum) -> Shader {
        let shader_string = get_shader_string_from_file(file_path);
        Shader {id: compile_shader(&shader_string,shader_type)}
    }
}

pub fn get_shader_string_from_file(file_path : &str ) -> String {
     fs::read_to_string(file_path).expect("failed to read file")
}

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        // Create GLSL shaders
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}