use glium::texture::texture2d::Texture2d;
use glium::Surface;
// mod shader;

#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, pos, tex_coords);

struct Hex {
    
}