use std::collections::HashMap;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton};

pub const MOUSE_SPEED: f32 = 10.;

#[derive(PartialEq)]
enum ButtonState {
    Clicked,
    Held,
}

impl ButtonState {
    pub fn is_clicked(&self) -> bool {
        match self {
            Self::Clicked => true,
            Self::Held => false,
        }
    }
    pub fn is_held(&self) -> bool {
        match self {
            Self::Clicked => false,
            Self::Held => true,
        }
    }
}

pub struct Mouse {
    button_states: HashMap<MouseButton, ButtonState>,
    moved: bool,
    delta_pos: (f32, f32),
    last_mouse_pos: PhysicalPosition<f64>,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            button_states: HashMap::new(),
            moved: false,
            delta_pos: (0., 0.),
            last_mouse_pos: PhysicalPosition::new(0., 0.),
        }
    }

    /// Should be called upon mouse button update
    pub fn update_buttons(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                // Insert on press
                self.button_states.insert(button, ButtonState::Clicked);
            }
            ElementState::Released => {
                // Remove on released
                self.button_states.remove(&button);
            }
        }
    }

    /// Should be called upon mouse cursor update
    pub fn update_cursor(&mut self, new_pos: PhysicalPosition<f64>) {
        self.moved = true;
        self.delta_pos = (
            self.last_mouse_pos.x as f32 - new_pos.x as f32,
            self.last_mouse_pos.y as f32 - new_pos.y as f32,
        );
        self.last_mouse_pos = new_pos;
    }

    /// Return whether the mouse has moved during the given update
    pub fn moved(&self) -> bool {
        self.moved
    }

    /// Get the delta position of the mouse
    pub fn delta_movement(&self) -> (f32, f32) {
        self.delta_pos
    }

    /// Return the last set position of the mouse
    pub fn last_pos(&self) -> PhysicalPosition<f64> {
        self.last_mouse_pos
    }

    /// Check whether a button is clicked
    pub fn clicked(&self, button: MouseButton) -> bool {
        if let Some(state) = self.button_states.get(&button) {
            state.is_clicked()
        } else {
            false
        }
    }

    /// Check wheter a button is held
    pub fn held(&self, button: MouseButton) -> bool {
        if let Some(state) = self.button_states.get(&button) {
            state.is_held()
        } else {
            false
        }
    }

    /// Must be called at the end of the game update
    pub fn refresh(&mut self) {
        for (_, state) in self.button_states.iter_mut() {
            if state == &ButtonState::Clicked {
                *state = ButtonState::Held;
            }
        }
        // Mouse is no longer moving
        self.moved = false;
    }
}
