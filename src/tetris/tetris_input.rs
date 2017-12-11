
use ::core::BinaryAxis;
use piston_window::{Input, Button, Key};

pub struct TetrisInput {
    pub arrow_left: BinaryAxis,
    pub arrow_right: BinaryAxis,
    pub arrow_up: BinaryAxis,
    pub arrow_down: BinaryAxis,
}

impl TetrisInput {
    pub fn new() -> Self {
        Self {
            arrow_left: BinaryAxis::new(),
            arrow_right: BinaryAxis::new(),
            arrow_up: BinaryAxis::new(),
            arrow_down: BinaryAxis::new(),
        }
    }

    pub fn update(&mut self, dt: f32, input_events: &[Input]) {
        // loop through all of the input events that happened this frame
        for entry in input_events {
            if let &Input::Button(button_input) = entry {
                match button_input.button {
                    Button::Keyboard(Key::Left) => self.arrow_left.state_change(&button_input.state),
                    Button::Keyboard(Key::Right) => self.arrow_right.state_change(&button_input.state),
                    Button::Keyboard(Key::Up) => self.arrow_up.state_change(&button_input.state),
                    Button::Keyboard(Key::Down) => self.arrow_down.state_change(&button_input.state),
                    _ => {}
                }
            }
        }

        //now update our axes with the elta time of the frame
        self.arrow_left.update(dt);
        self.arrow_right.update(dt);
        self.arrow_up.update(dt);
        self.arrow_down.update(dt);
    }
}