use std::f32::consts::PI;
use std::path::Path;
use std::time::Instant;

//Alias the backend to something less mouthful
use egui_backend::egui::{Color32, FullOutput};
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, gl, sdl2};
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use egui_sdl2_gl as egui_backend;
use egui_sdl2_gl::gl::types::GLfloat;
use sdl2::video::SwapInterval;

mod grid;
mod shader;
mod camera;
mod egui_manager;
mod light;
mod texture;
mod model;
mod mesh;

const SCREEN_WIDTH: u32 = 1000;
const SCREEN_HEIGHT: u32 = 800;
const MAX_TESSELLATION: u32 = 64;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);

    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_framebuffer_srgb_compatible(true);

    // OpenGL 3.2 is the minimum that we will support.
    gl_attr.set_context_version(3, 2);

    let window = video_subsystem
        .window(
            "scene-rendering",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Create a window context
    let _ctx = window.gl_create_context().unwrap();
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 2));

    // Enable vsync
    match window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::VSync) {
        Result::Ok(()) => {()},
        Result::Err(_) => {}
    }
    // Init egui stuff
    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let egui_ctx = egui::Context::default();
    let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();

    // grid variables
    let mut tessellation_level: u32 = 10;
    let mut z_coords : [GLfloat;16] = [0.0, 0.5, -0.5, 0.3, 0.2, 0.0, 0.4, 0.2, 0.1, - 0.1,
                                        -0.4, 0.3, 0.4, 0.2, 0.0,-0.3];
    let start_time = Instant::now();
    let mut object_color = Color32::WHITE;
    let mut light_color = Color32::WHITE;
    let mut is_light_moving = false;
    let mut light_z = 0.5;
    let mut kd = 0.5;
    let mut texture_set = false;
    let mut normal_set = false;
    let mut ks = 0.5;
    let mut m = 30;

    let mut fill = true;
    let mut mesh = true;
    let mut reflectors = true;
    let mut main_light = true;

    let mut grid = grid::Grid::new();
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }
    let mut quit = false;

    'running: loop {
        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        // An example of how OpenGL can be used to draw custom stuff with egui
        // overlaying it:
        // First clear the background to something nice.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT| gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.3, 0.2, 0.3, 1.0);
        }

        //update tessellation
        if grid.tessellation_level != tessellation_level {
            grid.tessellation_level = tessellation_level;
        }
        if grid.light.is_moving != is_light_moving {
            if is_light_moving {grid.light.start_moving()} else {grid.light.stop_moving()}
        }
        
        if grid.color != object_color && texture_set == false {grid.color = object_color}
        if grid.light.light_color != light_color  {grid.light.light_color = light_color};
        grid.mesh = mesh;
        grid.fill = fill;
        grid.main_light = main_light;
        grid.reflectors = reflectors;


        unsafe {
            if texture_set && grid.texture.is_none() {
                grid.add_texture(&Path::new("resources/images/texture.jpg"));
                grid.color = Color32::WHITE;
            }
            if texture_set == false {grid.remove_texture()}

            if normal_set && grid.normal_map.is_none() {
                grid.add_normal_map(&Path::new("resources/images/normal.jpg"));
            }
            if normal_set == false {grid.remove_normal_map()}

        }

        grid.light.update_light(ks,kd,m,light_z);
        grid.update_z_coords(z_coords);
        grid.light.update_light_pos();
        grid.angle = grid.angle + 0.005 % 360.0;

        grid.draw();

        egui::Window::new("settings").show(&egui_ctx, |ui| {
            ui.add(egui::Slider::new(&mut tessellation_level,1..=MAX_TESSELLATION).text("tessellation level"));
            egui_manager::add_sliders_to_egui(ui, &mut z_coords);
            ui.add(egui::Checkbox::new(&mut is_light_moving,"light animation"));
            egui_manager::add_color_pickers_to_egui(ui,&mut object_color,&mut light_color);
            egui_manager::add_light_sliders_to_egui(ui,&mut ks, &mut kd, &mut m, &mut light_z);
            ui.horizontal(|ui| {
                ui.radio_value(&mut texture_set, true, "texture");
                ui.radio_value(&mut texture_set, false, "color");
            });
            ui.add(egui::Checkbox::new(&mut normal_set,"set normal map"));
            ui.add(egui::Checkbox::new(&mut fill,"FILL"));
            ui.add(egui::Checkbox::new(&mut mesh,"MESH"));
            ui.add(egui::Checkbox::new(&mut main_light,"main_light"));
            ui.add(egui::Checkbox::new(&mut reflectors,"reflectors"));


            if ui.button("Quit").clicked() {
                quit = true;
            }
        });

        let FullOutput {
            platform_output,
            repaint_after,
            textures_delta,
            shapes,
        } = egui_ctx.end_frame();
        // Process output
        egui_state.process_output(&window, &platform_output);

        let paint_jobs = egui_ctx.tessellate(shapes);
        painter.paint_jobs(None, textures_delta, paint_jobs);
        window.gl_swap_window();


        if !repaint_after.is_zero() {
            if let Some(event) = event_pump.wait_event_timeout(5) {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {keycode :Some(key),..} => { grid.camera.process_key(key) }
                    _ => {
                        // Process input event
                        egui_state.process_input(&window, event, &mut painter);
                    }
                }
            }
        } else {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {
                        // Process input event
                        egui_state.process_input(&window, event, &mut painter);
                    }
                }
            }
        }

        if quit {
            break;
        }
    }
}

