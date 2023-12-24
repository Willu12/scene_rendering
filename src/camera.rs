use glam::{Mat4, Vec3};
use sdl2::keyboard::Keycode;

const CAMERA_SPEED : f32 = 2.0 * std::f32::consts::PI/360.0;
pub struct Camera {
    pub position : Vec3,
    target : Vec3,
    direction : Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {position: Vec3::new(0.1,1.0,2.0), target: Vec3::new(0.0,0.0,0.0), direction: Vec3::new(0.0,0.0,1.0)}
    }

    fn get_spherical_coordinates(&self) -> (f32,f32,f32) {
        let position = self.position;
        let r = (position.dot(position)).sqrt();
        let theta = f32::asin(position.z / r);
        let phi = f32::atan(position.y/ position.x);

        return (r,theta,phi);
    }

    fn update_position_from_spherical_coordinates(&mut self, r: f32, theta : f32, phi: f32) {
        self.position = get_position_from_spherical_coordinates(r,theta,phi);
    }

    pub fn rotate_vertical (&mut self, angle: f32) {
        let spherical = self.get_spherical_coordinates();
        self.update_position_from_spherical_coordinates(spherical.0,spherical.1 + angle, spherical.2);
    }

    pub fn rotate_horizontal(&mut self, angle : f32) {
        let spherical = self.get_spherical_coordinates();
        self.update_position_from_spherical_coordinates(spherical.0,spherical.1, spherical.2 + angle);
    }

    pub fn process_key(&mut self, key : sdl2::keyboard::Keycode) {
        match key  {
            Keycode::Down => self.rotate_vertical(CAMERA_SPEED),
            Keycode::Up => self.rotate_vertical(-CAMERA_SPEED),
            Keycode::Left => self.rotate_horizontal(-CAMERA_SPEED),
            Keycode::Right => self.rotate_horizontal(CAMERA_SPEED),
            _ => {},
        }
    }

    pub fn get_view(&self) -> Mat4 {
        glam::Mat4::look_at_lh(self.position,self.target,self.direction)
    }
}

fn get_position_from_spherical_coordinates(r : f32, theta:f32,phi : f32) -> Vec3 {
    let x = r * f32::cos(theta) * f32::cos(phi);
    let y = r * f32::cos(theta) * f32::sin(phi);
    let z = r * f32::sin(theta);

    Vec3::new(x,y,z)
}