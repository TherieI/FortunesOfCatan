use glium::Frame;
use glium::backend::Facade;

/// A panel that is drawn to the screen
pub trait Interface {
    /// Draw the interface
    fn draw<F>(&self, facade: &F, frame: Frame) -> Frame
    where
        F: ?Sized + Facade;
}

/// Draw information about the user's hand
pub struct Hand {

}