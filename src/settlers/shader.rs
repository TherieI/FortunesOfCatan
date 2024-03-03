use glium::Texture2d;
use glium::{backend::Facade, Program};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;

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
        facade: &F,
        name: &'a str,
        vertex_path: &'static str,
        fragment_path: &'static str,
        geometry_path: Option<&'static str>,
    ) -> Result<(), Box<dyn Error>> {
        let vertex_src = fs::read_to_string(vertex_path)?;
        let fragment_src = fs::read_to_string(fragment_path)?;
        let program = if let Some(geometry_src) = geometry_path {
            let geometry_src = fs::read_to_string(geometry_src)?;
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

pub struct TextureManager<'a> {
    textures: HashMap<&'a str, Texture2d>,
}

impl<'a> TextureManager<'a> {
    // Create a shader program manager
    pub fn new() -> Self {
        TextureManager {
            textures: HashMap::with_capacity(5),
        }
    }

    /// Compile a Texture for the manager
    pub fn add_texture<F: Sized + Facade>(
        &mut self,
        facade: &F,
        name: &'a str,
        image_path: &'static str,
    ) -> Result<(), Box<dyn Error>> {
        // Generate texture for chances
        let image = image::load(
            BufReader::new(File::open(image_path)?),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(facade, image).unwrap();

        self.textures.entry(name).or_insert(texture);
        Ok(())
    }

    pub fn texture(&self, name: &'a str) -> Option<&Texture2d> {
        self.textures.get(name)
    }
}
