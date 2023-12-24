use egui_sdl2_gl::egui::Color32;
use glam::Vec3;


const LIGHT_SPEED : f32 = std::f32::consts::PI/360.0;
pub struct Light {
    pub kd: f32,
    pub ks: f32,
    pub light_color: Color32,
    pub m: u32,
    pub light_position: Vec3,
    pub is_moving: bool,
}

impl Light {
    pub fn new() -> Light {
        let kd = 0.8;
        let ks = 0.8;
        let light_color = Color32::WHITE;
        let m = 70;
        let light_position = Vec3::new(0.0,0.0,0.5);
        Light {kd,ks,light_color,m,light_position,is_moving: false}
    }

    pub fn start_moving(&mut self) {
        self.is_moving = true;
        let r = 0.5;
        let phi = 0.0;
        self.update_pos_from_polar(r,phi);
    }

    pub fn stop_moving(&mut self) {
        self.is_moving = false;
        self.light_position = Vec3::new(0.0,0.0,0.5);
    }

    fn update_pos_from_polar(&mut self, r: f32, phi: f32) {
        self.light_position.x = r * f32::cos(phi);
        self.light_position.y = r * f32::sin(phi);
    }
    pub fn update_light_pos(&mut self) {
        if self.is_moving == false {return}

        let (r, mut phi) = self.get_polar_coordinates();
        phi = phi + LIGHT_SPEED;
        if phi > 2.0 * std::f32::consts::PI {phi = phi - 2.0* std::f32::consts::PI}
        self.update_pos_from_polar(r,phi);
    }

    fn get_polar_coordinates(&self) -> (f32,f32) {
        let r = 0.5;
        let mut phi  = 0.0;
        let x  = self.light_position.x;
        let y = self.light_position.y;
        if x > 0.0 && y >= 0.0 {phi = f32::atan(y/x)}
        if x > 0.0 && y < 0.0 {phi = f32::atan(y/x) + 2.0 * std::f32::consts::PI}
        if x < 0.0 {phi = f32::atan(y/x) + std::f32::consts::PI}
        if x == 0.0  && y > 0.0 { phi = std::f32::consts::PI/2.0}
        if x == 0.0  && y < 0.0 { phi = 3.0 * std::f32::consts::PI/2.0}

        return (r,phi)
    }

    pub fn update_light(&mut self,ks: f32, kd: f32, m:u32, z_coord: f32) {
        self.m = m;
        self.ks = ks;
        self.kd = kd;
        self.light_position.z = z_coord;
    }

}