use glium::{backend::Facade, Program};
use std::collections::HashMap;
use std::fs;

pub struct ProgramManager<'a> {
    programs: HashMap<&'a str, Program>,
}

impl<'a> ProgramManager<'a> {
    // Create a shader program manager
    pub fn new() -> Self {
        ProgramManager {
            programs: HashMap::with_capacity(5),
        }
    }

    /// Compile a program for the manager
    pub fn add_program<F: Sized + Facade>(
        &mut self,
        name: &'a str,
        facade: &F,
        vertex_path: &'static str,
        fragment_path: &'static str,
        geometry_path: Option<&'static str>,
    ) -> Result<(), glium::ProgramCreationError> {
        let vertex_src = fs::read_to_string(vertex_path).expect("Vertex shader not found.");
        let fragment_src = fs::read_to_string(fragment_path).expect("Fragment shader not found.");
        let program = if let Some(geometry_src) = geometry_path {
            let geometry_src =
                fs::read_to_string(geometry_src).expect("Geometry shader not found.");
            glium::Program::from_source(facade, &vertex_src, &fragment_src, Some(&geometry_src))
        } else {
            glium::Program::from_source(facade, &vertex_src, &fragment_src, None)
        }?;
        self.programs.entry(name).or_insert(program);
        Ok(())
    }

    pub fn program(&self, name: &'a str) -> Option<&Program> {
        self.programs.get(name)
    }
}
