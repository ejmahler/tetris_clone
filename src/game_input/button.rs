
use piston_window::ButtonState;

pub struct BinaryAxis {
    currently_held: bool,

    frames_in_state: u32,
    time_in_state: f32,

    frames_in_previous: u32,
    time_in_previous: f32,
}

impl BinaryAxis {
    pub fn new() -> Self {
        Self {
            currently_held: false,
            frames_in_state: 0,
            time_in_state: 0.0,

            frames_in_previous: 0,
            time_in_previous: 0.0,
        }
    }

    pub fn state_change(&mut self, state: &ButtonState) {
        let new_state = match state {
            &ButtonState::Press => true,
            &ButtonState::Release => false,
        };

        if self.currently_held != new_state {
            self.currently_held = new_state;

            self.frames_in_previous = self.frames_in_state;
            self.time_in_previous = self.time_in_state;

            self.frames_in_state = 0;
            self.time_in_state = 0.0;
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.frames_in_state += 1;
        self.time_in_state += dt;
    }

    pub fn pressed(&self) -> bool {
        self.currently_held
    }
    pub fn pressed_this_frame(&self) -> bool {
        self.pressed() && self.frames_in_state < 2
    }
}