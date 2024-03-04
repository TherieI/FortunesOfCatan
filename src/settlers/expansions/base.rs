use super::expansion::Expansion;
use crate::settlers::board::background::quad;
use crate::settlers::board::hex;
use crate::settlers::camera::Camera;
use crate::settlers::game::DeltaTime;
use crate::settlers::matrix::Mat4;
use crate::settlers::shader::{ProgramManager, TextureManager};
use crate::settlers::Board;
use glium::backend::Facade;
use glium::index::NoIndices;
use glium::uniforms::UniformBuffer;
use glium::{Frame, IndexBuffer, Surface, VertexBuffer};
use std::time::Instant;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase};

const MOUSE_SPEED: f32 = 10.;

pub struct Mouse {
    left_click_pressed: bool,
    last_mouse_pos: PhysicalPosition<f64>,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            left_click_pressed: false,
            last_mouse_pos: PhysicalPosition::new(0., 0.),
        }
    }

    pub fn update_buttons(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                if button == MouseButton::Left {
                    self.left_click_pressed = true;
                }
            }
            ElementState::Released => {
                if button == MouseButton::Left {
                    self.left_click_pressed = false;
                }
            }
        }
    }

    pub fn update_cursor(&mut self, normal_pos: PhysicalPosition<f64>) {
        self.last_mouse_pos = normal_pos;
    }

    pub fn left_pressed(&self) -> bool {
        self.left_click_pressed
    }

    pub fn last_pos(&self) -> PhysicalPosition<f64> {
        self.last_mouse_pos
    }
}

pub struct BaseGame<'p> {
    window_dim: PhysicalSize<u32>,
    // Keep track of the program time
    time: Instant,
    board: Board,
    program_manager: ProgramManager<'p>,
    // Texture for the hex's
    texture_manager: TextureManager<'p>,
    camera: Camera,
    delta_time: DeltaTime,
    mouse: Mouse,
    scale: f32,
}

impl<'p> BaseGame<'p> {
    pub fn new<F>(facade: &F) -> Self
    where
        F: Sized + Facade,
    {
        // Generate board
        let mut board: Board = Board::from_file("src/settlers/board/maps/chungus.focm").unwrap();
        board.randomize();
        // Manage textures
        let mut texture_manager = TextureManager::new();
        // Generate texture for hex tiles
        texture_manager
            .add_texture(facade, "hex_tm", "assets/hex/hex_tilemap.png")
            .expect("hex texture should be found");
        // Generate texture for chances
        texture_manager
            .add_texture(facade, "chance_tm", "assets/hex/chances_tilemap.png")
            .expect("chances texture should be found");
        // Generate texture for structures
        texture_manager
            .add_texture(facade, "building_tm", "assets/structures/settlement.png")
            .expect("structure texture should be found");
        // Create shader program
        use crate::settings::WINDOW_DEFAULT_SIZE;
        let mut program_manager = ProgramManager::new();
        program_manager
            .add_program(
                facade,
                "hex",
                "glsl/hex/hex.v.glsl",
                "glsl/hex/hex.f.glsl",
                Some("glsl/hex/hex.g.glsl"),
            )
            .expect("Hex shaders properly compiling");
        program_manager
            .add_program(facade, "bg", "glsl/bg/bg.v.glsl", "glsl/bg/bg.f.glsl", None)
            .expect("Background shaders properly compiling");
        program_manager
            .add_program(
                facade,
                "structures",
                "glsl/structure/str.v.glsl",
                "glsl/structure/str.f.glsl",
                Some("glsl/structure/str.g.glsl"),
            )
            .expect("Structure shaders properly compiling");

        Self {
            window_dim: WINDOW_DEFAULT_SIZE,
            time: Instant::now(),
            board,
            program_manager,
            texture_manager,
            camera: Camera::new(8., 0.),
            delta_time: DeltaTime::new(),
            mouse: Mouse::new(),
            scale: 0.13,
        }
    }

    fn mvp(&self) -> [[f32; 4]; 4] {
        let mut projection = Mat4::projection(
            self.window_dim.width as f32 / self.window_dim.height as f32,
            90.,
            1.0,
            -1.0,
        );
        let mut transform = Mat4::identity();
        transform
            .translate(-0.4, -0.6, 0.)
            .scale_uniformly(self.scale);
        let view = self.camera.view_matrix();
        projection.multiply_by(&view).multiply_by(&transform);
        // println!("{}", projection);
        projection.to_array()
    }
}

impl<'p> Expansion for BaseGame<'p> {
    // Called on mouse move
    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        let last_pos = self.mouse.last_pos();
        if self.mouse.left_pressed() {
            self.camera.move_to(|x, y, z| {
                (
                    x - (last_pos.x - position.x) as f32 * MOUSE_SPEED
                        / self.window_dim.width as f32,
                    y - (last_pos.y - position.y) as f32 * MOUSE_SPEED
                        / self.window_dim.height as f32,
                    z,
                )
            })
        }
        self.mouse.update_cursor(position);
    }

    // Called on recieving mouse input
    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        self.mouse.update_buttons(state, button);
    }

    fn scroll_input(&mut self, delta: MouseScrollDelta, _phase: TouchPhase) {
        match delta {
            MouseScrollDelta::LineDelta(_, scroll) => self.scale += scroll / 200.,
            _ => (),
        }
    }

    // Called on recieving keyboard input
    fn keyboard_input(&mut self, event: KeyEvent) {
        use winit::keyboard::{KeyCode, PhysicalKey};
        let move_const = 100.;
        match &event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => self
                .camera
                .move_to(|x, y, z| (x, y + move_const * self.delta_time.delta(), z)),
            _ => (),
        }
    }

    fn window_size(&mut self, new_size: PhysicalSize<u32>) {
        println!("Scale [{}, {}]", new_size.width, new_size.height);
        self.window_dim = new_size;
    }

    // Called every time before draw
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.delta_time.update();
        Ok(())
    }

    fn draw<F>(&self, facade: &F, mut frame: Frame) -> Frame
    where
        F: ?Sized + Facade,
    {
        use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
        use glium::BlendingFunction::Addition;
        use glium::LinearBlendingFactor::{OneMinusSourceAlpha, SourceAlpha};
        let mvp = self.mvp();
        // ============== Background ===============
        let mut total_hex: u32 = 0;
        // The board must have less than 64 total hex tiles
        let hex_positions: [(f32, f32); hex::MAX_HEX as usize] = {
            let mut arr = [(0.0, 0.0); hex::MAX_HEX as usize];
            self.board
                .hex_buffers()
                .iter()
                .map(|vert| vert.position())
                .enumerate()
                .for_each(|(i, (x, y))| {
                    // Matrix transformation with camera position (Can't explain the +1 but its needed)
                    arr[i] = (
                        x * mvp[0][0] + y * mvp[0][1] + mvp[0][3] + mvp[3][0] + 1.,
                        x * mvp[1][0] + y * mvp[1][1] + mvp[1][3] + mvp[3][1] + 1.,
                    );
                    total_hex += 1;
                });
            arr
        };
        let hex_pos_buffer = UniformBuffer::new(facade, hex_positions).unwrap();
        let bg_vbo = VertexBuffer::new(facade, &quad::VERTICES).unwrap();
        let bg_ebo = IndexBuffer::new(
            facade,
            glium::index::PrimitiveType::TrianglesList,
            &quad::INDICES,
        )
        .unwrap();
        frame
            .draw(
                &bg_vbo,
                &bg_ebo,
                &self
                    .program_manager
                    .program("bg")
                    .expect("Background program exists"),
                &uniform! {
                    mvp: mvp,
                    total_hex: total_hex,
                    hex_positions: &hex_pos_buffer,
                    u_scale: self.scale,
                    u_resolution: (self.window_dim.width, self.window_dim.height),
                    u_time: self.time.elapsed().as_secs_f32(),
                },
                &Default::default(),
            )
            .unwrap();

        // ============== Hex tiles ================
        let vertices = self.board.hex_buffers();
        let vertex_buffer = VertexBuffer::new(facade, &vertices).unwrap();
        let index_buffer = NoIndices(glium::index::PrimitiveType::Points);

        // For png transparency
        let params = glium::DrawParameters {
            blend: glium::Blend {
                color: Addition {
                    source: SourceAlpha,
                    destination: OneMinusSourceAlpha,
                },
                alpha: Addition {
                    source: SourceAlpha,
                    destination: OneMinusSourceAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },
            ..Default::default()
        };
        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self
                    .program_manager
                    .program("hex")
                    .expect("Hex program exists"),
                &uniform! { u_mvp: mvp,
                    u_resolution: (self.window_dim.width, self.window_dim.height),
                    u_time: self.time.elapsed().as_secs_f32(),
                    texture_map_hex: self.texture_manager.texture("hex_tm").unwrap().sampled()
                        .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp)
                        .magnify_filter(MagnifySamplerFilter::Nearest)
                        .minify_filter(MinifySamplerFilter::Nearest),
                    texture_map_chances: self.texture_manager.texture("chance_tm").unwrap().sampled()
                        .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp)
                        .magnify_filter(MagnifySamplerFilter::Nearest)
                        .minify_filter(MinifySamplerFilter::Nearest),
                },
                &params,
            )
            .unwrap();

        // =============== Settlements / Cities / Roads ==================
        let vertices = self.board.building_buffers();
        let vertex_buffer = VertexBuffer::new(facade, &vertices).unwrap();
        let index_buffer = NoIndices(glium::index::PrimitiveType::Points);
        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self
                    .program_manager
                    .program("structures")
                    .expect("structure program exists"),
                &uniform! { u_mvp: mvp,
                    u_resolution: (self.window_dim.width, self.window_dim.height),
                    u_time: self.time.elapsed().as_secs_f32(),
                    texture_map_structures: self.texture_manager.texture("building_tm").unwrap().sampled()
                        .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp)
                        .magnify_filter(MagnifySamplerFilter::Nearest)
                        .minify_filter(MinifySamplerFilter::Nearest),
                },
                &params,
            )
            .unwrap();
        frame
    }
}
