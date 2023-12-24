use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use std::mem;
use std::path::Path;
use std::ptr;
use std::str;
use glam::{Mat3, Mat4, Vec3, Vec4Swizzles};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::camera::Camera;
use crate::light::Light;
use crate::shader::*;
use egui_sdl2_gl::egui::Color32;
use crate::texture::Texture;


pub struct Grid {
    pub tessellation_level: u32,
    pub vertices: [GLfloat; 48],
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
    pub program: GLuint,
    pub mesh_program: GLuint,
    pub color: Color32,
    pub camera : Camera,
    pub light : Light,
    pub texture: Option<Texture>,
    pub normal_map: Option<Texture>,
    pub fill : bool,
    pub mesh : bool,
    pub main_light : bool,
    pub reflectors: bool,
    pub angle: f32,
}

impl Grid {
    pub fn new() -> Self {

        let mut vertices = Grid::create_patch_vertices();

        let (mut vao,mut vbo,mut ebo) = (0,0,0);
        let vs = Shader::new(VS_SRC, gl::VERTEX_SHADER);
        let fs = Shader::new(FS_SRC, gl::FRAGMENT_SHADER);
        let tcs = Shader::new(TCS_SRC,gl::TESS_CONTROL_SHADER);
        let tes = Shader::new(TES_SRC,gl::TESS_EVALUATION_SHADER);

        let fsm = Shader::new(FSM_SRC,gl::FRAGMENT_SHADER);
        let vsm = Shader::new(VS_SRC, gl::VERTEX_SHADER);
        let tcsm = Shader::new(TCS_SRC,gl::TESS_CONTROL_SHADER);
        let tesm = Shader::new(TES_SRC,gl::TESS_EVALUATION_SHADER);

        let program = link_program(vs.id, fs.id,tcs.id,tes.id);
        let mesh_program = link_program(vsm.id,fsm.id,tcsm.id,tesm.id);
        let color = Color32::WHITE;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1,&mut ebo);
        }

        let camera = Camera::new();
        let light = Light::new();
        vertices[7 * 3 + 2] = 0.5;
        let tessellation_level = 1;
        let texture = None;
        let normal_map = None;

        let grid = Grid {tessellation_level, vertices, vao, vbo, ebo, program,mesh_program, color, camera,light,texture,normal_map, fill: true, mesh : true, reflectors:true,main_light:true,angle: 0.0};
        grid.init_grid();

        unsafe {
            gl::UseProgram(program);

            let text_map_location = get_uniform_location(program,"ourTexture");
            let normal_map_location = get_uniform_location(program,"normalMap");

            gl::Uniform1i(text_map_location,0);
            gl::Uniform1i(normal_map_location,1);
        }

        return grid;
    }

    pub fn update_z_coords(&mut self, z_coords : [GLfloat;16]) {

        let mut changed = false;
        for i in 0..16 {
            if z_coords[i] != self.vertices[i * 3 + 2] {
                self.vertices[i * 3 + 2 ] = z_coords[i];
                changed = true;
            }
        }
        if changed {self.init_grid()};
    }

    fn create_patch_vertices() -> [GLfloat;48] {
        let n = 4;
        let stride = 1.5 / (n - 1) as f32;

        let mut vertices : Vec<GLfloat> = vec![];

        for i in 0..n {
            for j in 0..n {
                vertices.push(-0.75 + stride * j as f32);
                vertices.push(-0.75 + stride * i as f32);
                vertices.push(0.0);
            }
        }
        let array = match vertices.try_into() {
            Ok(ba) => ba,
            Err(_) => panic!("Expected a Vec of length {} but it was different", 48),
        };
        return array;
    }

    fn get_color_from_color32(color: Color32) -> [GLfloat;3] {
        [color.r() as f32 /255.0, color.g() as f32 /255.0, color.b() as f32 /255.0]
    }
   pub fn init_grid(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.vertices[0]),
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0 as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );

            gl::EnableVertexAttribArray(0 as GLuint);
            gl::PatchParameteri(gl::PATCH_VERTICES,16);
        }
   }

    pub unsafe fn add_texture(&mut self, path: &Path) {
        self.texture = Some(Texture::new());
        self.texture.as_ref().unwrap().load(path);
    }

    pub unsafe fn add_normal_map(&mut self, path: &Path) {
        self.normal_map = Some(Texture::new());
        self.normal_map.as_ref().unwrap().load(path);
    }

    unsafe fn set_mesh_uniforms(&self)  {
        let tessellation_level_location = get_uniform_location(self.mesh_program,"TessLevel");
        self.set_matrices(self.mesh_program);
        gl::Uniform1ui(tessellation_level_location,self.tessellation_level);
    }


    unsafe fn set_uniforms(&self) {
        unsafe {
            let light_pos_location = get_uniform_location(self.program,"lightPos");
            let camera_pos_location = get_uniform_location(self.program,"cameraPos");
            let object_color_location = get_uniform_location(self.program,"objectColor");
            let light_color_location = get_uniform_location(self.program,"lightColor");
            let kd_location = get_uniform_location(self.program,"kd");
            let ks_location = get_uniform_location(self.program,"ks");
            let m_location = get_uniform_location(self.program,"m");
            let main_light = get_uniform_location(self.program,"main_light");
            let reflectors = get_uniform_location(self.program,"reflectors");

            gl::Uniform1i(reflectors,self.reflectors as i32);
            gl::Uniform1i(main_light,self.main_light as i32);

            let tessellation_level_location = get_uniform_location(self.program,"TessLevel");

            self.set_matrices(self.program);
            gl::Uniform1ui(tessellation_level_location,self.tessellation_level);

            let light_pos = self.light.light_position;
            let camera_pos = self.camera.position;
            let light_col = Grid::get_color_from_color32(self.light.light_color);
            let object_col = Grid::get_color_from_color32(self.color);

            gl::Uniform3f(light_pos_location,light_pos[0],light_pos[1],light_pos[2]);
            gl::Uniform3f(camera_pos_location,camera_pos[0],camera_pos[1],camera_pos[2]);
            gl::Uniform3f(light_color_location,light_col[0],light_col[1],light_col[2]);
            gl::Uniform3f(object_color_location,object_col[0],object_col[1],object_col[2]);
            gl::Uniform1f(kd_location,self.light.kd);
            gl::Uniform1f(ks_location,self.light.ks);
            gl::Uniform1ui(m_location,self.light.m);
        }
    }

    fn set_matrices(&self,program: GLuint) {
        let view = self.camera.get_view();
        let projection =glam::Mat4::perspective_lh(
            std::f32::consts::PI / 2.0,
            SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
            0.1,
            100.0);

        let modal = Mat4::from_rotation_x(self.angle);


        let mvp = projection * view * modal;
        let normal = Mat3::from_cols(view.x_axis.xyz(),view.y_axis.xyz(),view.z_axis.xyz());

        unsafe {
            let mvp_location = get_uniform_location(program,"MVP");
            let view_location = get_uniform_location(program,"ModelViewMatrix");
            let normal_location = get_uniform_location(program,"NormalMatrix");
            gl::UniformMatrix4fv(mvp_location,1,gl::FALSE,mvp.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(view_location,1,gl::FALSE,view.to_cols_array().as_ptr());
            gl::UniformMatrix3fv(normal_location,1,gl::FALSE,normal.to_cols_array().as_ptr());
        }
    }

    unsafe fn prepare_textures(&self) {

        let is_texture_set_location = get_uniform_location(self.program,"isTextureSet");
        match self.texture.as_ref() {
            Some(texture) => {
                gl::Uniform1i(is_texture_set_location,true as i32);
                texture.activate(gl::TEXTURE0);

            }
            None => {
                gl::Uniform1i(is_texture_set_location,false as i32);
            }
        }
    }

    unsafe fn prepare_normal_map(&self) {
        let is_normal_set_location = get_uniform_location(self.program,"isNormalMapSet");

        match self.normal_map.as_ref() {
            Some(normal_map) => {
                gl::Uniform1i(is_normal_set_location,true as i32);
                normal_map.activate(gl::TEXTURE1);
            }
            None => {
                gl::Uniform1i(is_normal_set_location,false as i32);
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DepthFunc(gl::LESS);
            gl::BindVertexArray(self.vao);
                if self.fill {
                    gl::UseProgram(self.program);
                    self.set_uniforms();
                    self.prepare_textures();
                    self.prepare_normal_map();
                    gl::DrawArrays(gl::PATCHES,0,16);
                }

                if self.mesh {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

                    gl::UseProgram(self.mesh_program);
                    self.set_mesh_uniforms();
                    self.set_uniforms();

                    gl::DrawArrays(gl::PATCHES,0,16);
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }


            gl::DepthFunc(gl::ALWAYS);
            gl::BindVertexArray(0);
        }
    }

    pub fn remove_texture(&mut self) {
        self.texture = None;
    }
    pub fn remove_normal_map(&mut self) {self.normal_map = None;}
}

unsafe fn get_uniform_location(program: GLuint, uniform_name: &str) -> GLint {
    let cname = std::ffi::CString::new(uniform_name).expect("CString::new failed");

    unsafe {
      gl::GetUniformLocation(program,cname.as_ptr())
    }
}

pub fn link_program(vs: GLuint, fs: GLuint,tcs: GLuint, tes: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::AttachShader(program,tcs);
        gl::AttachShader(program,tes);
        gl::LinkProgram(program);

        gl::DetachShader(program, fs);
        gl::DetachShader(program, vs);
        gl::DetachShader(program, tcs);
        gl::DetachShader(program,tes);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteShader(tcs);
        gl::DeleteShader(tes);

        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

