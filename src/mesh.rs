use std::ops::Add;
use std::ptr;
use glam::{Vec2, Vec3};
use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::{GLsizei, GLvoid};
use crate::grid::get_uniform_location;
use crate::shader::Shader;


pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    tex_coords: Vec2
}

pub struct Texture {
    id: u32,
    texture_type: String
}
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    vao: u32,
    vbo: u32,
    ebo: u32
}

impl Mesh {

    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {

        let mut mesh = Mesh{ vertices,indices,textures,vao: 0, vbo: 0,ebo: 0};
        mesh.setup_mesh();

        mesh
    }

    pub fn draw(&self,shader: &Shader) {
        let mut diffuse_nr: u32 = 0;
        let mut specular_nr: u32 = 0;

        for (i,texture) in self.textures.iter().enumerate() {

            let number = match &texture.texture_type[..] {
                "texture_diffuse" => {diffuse_nr+=1; diffuse_nr.to_string()},
                "texture_specular" => {specular_nr+=1; specular_nr.to_string()},
                _ => "0".to_string()
            };

            if number == "0" {continue};
            unsafe {
                gl::Uniform1i(get_uniform_location(shader.id,texture.texture_type.clone().add(number.as_str()).as_str()),
                i as gl::types::GLint);

                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }

            //draw mesh
            unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawElements(gl::TRIANGLES, self.indices.len() as GLsizei, gl::UNSIGNED_INT, ptr::null_mut());
                gl::BindVertexArray(0);
                gl::ActiveTexture(gl::TEXTURE0);
            }
        }
    }

    fn setup_mesh(&mut self) {
        unsafe {
            gl::GenVertexArrays(1,&mut self.vao);
            gl::GenBuffers(1,&mut self.vbo);
            gl::GenBuffers(1,&mut self.ebo);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER,self.vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                std::mem::transmute(&self.vertices[0]),
                gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                    std::mem::transmute(&self.indices[0]),
                gl::STATIC_DRAW);

            // vertex positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    std::mem::size_of::<Vertex> as GLsizei,
                                    ptr::null()
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    std::mem::size_of::<Vertex> as GLsizei,
                                    (ptr::null::<Vertex>() as *const GLvoid)
                                        .offset(memoffset::offset_of!(Vertex, normal) as isize),
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    std::mem::size_of::<Vertex> as GLsizei,
                                    (ptr::null::<Vertex>() as *const GLvoid)
                                        .offset(memoffset::offset_of!(Vertex, tex_coords) as isize),
            );

            gl::BindVertexArray(0);
        }
    }
}