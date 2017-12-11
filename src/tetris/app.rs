
use piston::event_loop::EventSettings;
use piston_window::{PistonWindow, RenderArgs, AfterRenderArgs, UpdateArgs, IdleArgs, Input, Event, Loop, rectangle, clear};
use num_traits;

use super::tetris_input::TetrisInput;

pub struct TetrisApp {
    window: PistonWindow,
    input: TetrisInput,
    red: f32,
    green: f32,
}

impl TetrisApp {
    pub fn new(window: PistonWindow) -> Self {
        Self {
            window,
            input: TetrisInput::new(),
            red: 0.0,
            green: 0.0,
        }
    }

    pub fn create_event_settings(&self) -> EventSettings {
        let mut settings = EventSettings::new();
        settings.max_fps = 30;
        settings.ups = 30;

        settings
    }

    pub fn get_window<'a>(&'a mut self) -> &'a mut PistonWindow {
        &mut self.window
    }




    pub fn render(&mut self, event: RenderArgs) {
        let placation_event = Event::from(Loop::from(event));
        let red = self.red;
        let green = self.green;

        self.window.draw_2d(&placation_event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([red, green, 0.0, 1.0], // red
                        [0.0, 0.0, 100.0, 100.0],
                        context.transform,
                        graphics);
        });
    }
    pub fn after_render(&mut self, _: AfterRenderArgs) {

    }

    pub fn update(&mut self, args: UpdateArgs) {
        let dt = args.dt as f32;

        // update our input axes
        self.input.update(dt);
        
        // if the player pressed the left arrow this frame only, set red to 1
        if self.input.arrow_left.pressed_this_frame() {
            self.red = 1.0;
        } else {
            self.red = num_traits::clamp(self.red - dt, 0.0, 1.0);
        }

        // if the player is holding the right arrow, fade to green
        if self.input.arrow_right.pressed() {
            self.green = num_traits::clamp(self.green + dt, 0.0, 1.0);
        } else {
            self.green = num_traits::clamp(self.green - dt, 0.0, 1.0);
        }
    }
    pub fn idle(&mut self, _: IdleArgs) {

    }
    pub fn handle_input(&mut self, args: Input) {
        self.input.handle_input(&args);
    }
}