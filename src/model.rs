use std::path::Path;
use russimp::mesh::Mesh;
use russimp::scene::{PostProcess, Scene};
use egui_sdl2_gl::gl;


pub struct Model {
    meshes: Vec<Mesh>,
    directory: String,
}

impl Model {
    pub fn new(path: &str) -> Self {
        Self::load_model(path)
    }

    fn load_model(path: &str) -> Self {
        let scene = Scene::from_file(path,
                                     vec![PostProcess::CalculateTangentSpace,
                                          PostProcess::Triangulate,
                                          PostProcess::JoinIdenticalVertices,
                                          PostProcess::SortByPrimitiveType]);
        let mut directory = String::new();
        let mut meshes = Vec::<Mesh>::new();

        match scene {
            Ok(scene) => {
                let path = Path::new(path);
                directory = path.parent().unwrap().to_str().unwrap_or("").to_string();

                for mesh in scene.meshes {
                    
                }
            }
            Err(err) => {
                panic!("{}",err)
            }
        }

        Model {meshes,directory}
    }

   // fn process_node(ai )
}