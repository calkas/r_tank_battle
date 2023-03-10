use piston_window::{G2d,math};
pub mod tank;

pub trait Entity {
    fn render(&self, view: math::Matrix2d, g: &mut G2d);
    
}