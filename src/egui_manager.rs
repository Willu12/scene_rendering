use egui_sdl2_gl::egui;
use egui_sdl2_gl::egui::Color32;
use egui_sdl2_gl::gl::types::GLfloat;

pub fn add_sliders_to_egui(ui: &mut egui_sdl2_gl::egui::Ui, z_coords : &mut [GLfloat; 16]) {
    let n = 4;
    egui::Grid::new("1").show(ui, |ui| {
        for i in 0..n {
            for j in 0..n {
                ui.add(egui::Slider::new(&mut z_coords[i * n + j],-0.5..=0.5));
            }
            ui.end_row();
        }
    });
}

pub fn add_color_pickers_to_egui(ui: &mut egui_sdl2_gl::egui::Ui, object_color: &mut Color32,light_color: &mut Color32) {
    egui::Grid::new("color_pickers").show(ui, |ui| {
        ui.label("object color");
        ui.color_edit_button_srgba(object_color);
        ui.label("light color");
        ui.color_edit_button_srgba(light_color);
    });
}

pub fn add_light_sliders_to_egui(ui: &mut egui_sdl2_gl::egui::Ui,ks: &mut f32, kd: &mut f32, m: &mut u32,z : &mut f32) {
    egui::Grid::new("color_sliders").show(ui, |ui| {
        ui.add(egui::Slider::new(ks,0.0..=1.0).text("ks"));
        ui.add(egui::Slider::new(kd,0.0..=1.0).text("kd"));
        ui.add(egui::Slider::new(m,1..=100).text("m"));
        ui.add(egui::Slider::new(z,0.0..=1.0).text("light_z"));

    });
}