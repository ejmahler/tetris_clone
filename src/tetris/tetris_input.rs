
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

    pub fn update(&mut self, dt: f32) {
        self.arrow_left.update(dt);
        self.arrow_right.update(dt);
        self.arrow_up.update(dt);
        self.arrow_down.update(dt);
    }

    pub fn handle_input(&mut self, input: &Input) {
        if let &Input::Button(args) = input {
            match args.button {
                Button::Keyboard(Key::Left) => self.arrow_left.state_change(&args.state),
                Button::Keyboard(Key::Right) => self.arrow_right.state_change(&args.state),
                Button::Keyboard(Key::Up) => self.arrow_up.state_change(&args.state),
                Button::Keyboard(Key::Down) => self.arrow_down.state_change(&args.state),
                _ => {}
            }
        }
    }
}