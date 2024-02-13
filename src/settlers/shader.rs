use glium::backend::Facade;
use std::fs;

pub fn create_program<'a, F: Sized + Facade>(
    facade: &F,
    vertex_path: &'a str,
    fragment_path: &'a str,
    geometry_path: Option<&'a str>,
) -> Result<glium::Program, glium::ProgramCreationError> {
    let vertex_src = fs::read_to_string(vertex_path).expect("Vertex shader not found.");
    let fragment_src = fs::read_to_string(fragment_path).expect("Fragment shader not found.");
    if let Some(geometry_src) = geometry_path {
        let geometry_src = fs::read_to_string(geometry_src).expect("Geometry shader not found.");
        glium::Program::from_source(facade, &vertex_src, &fragment_src, Some(&geometry_src))
    } else {
        glium::Program::from_source(facade, &vertex_src, &fragment_src, None)
    }
}
