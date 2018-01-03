
use piston::event_loop::EventSettings;
use piston_window::{PistonWindow, RenderArgs, AfterRenderArgs, UpdateArgs, IdleArgs, Input};
use num_traits;

use cgmath::{Transform, Vector3, Vector4, Quaternion, Decomposed};

use ::core::App;

use super::tetris_input::TetrisInput;
use super::render::RenderState;

pub struct TetrisApp<'app> {
    window: &'app mut PistonWindow,
    render_state: RenderState,
    input: TetrisInput,
    red: f32,
    green: f32,
}

impl<'app> TetrisApp<'app> {
    pub fn new(window: &'app mut PistonWindow) -> Self {
        let render_state = RenderState::new(window);
        Self {
            window,
            render_state: render_state,
            input: TetrisInput::new(),
            red: 0.0,
            green: 0.0,
        }
    }
}
impl<'app> App for TetrisApp<'app> {
    fn create_event_settings(&self) -> EventSettings {
        let mut settings = EventSettings::new();
        settings.max_fps = 30;
        settings.ups = 30;

        settings
    }

    fn get_window<'a>(&'a mut self) -> &'a mut PistonWindow {
        &mut self.window
    }




    fn render(&mut self, _: &RenderArgs) {
        let transform: Decomposed<Vector3<f32>, Quaternion<f32>> = Decomposed::one();
        let color = Vector4::new(self.red, self.green, 0.0, 1.0);

        self.render_state.render_tetris_block(&mut self.window, &transform, &color);
    }
    fn after_render(&mut self, _: &AfterRenderArgs) {

    }


    fn update(&mut self, args: &UpdateArgs, input_events: &[Input]) {
        let dt = args.dt as f32;

        // update our input axes
        self.input.update(dt, input_events);
        
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
    fn idle(&mut self, _: &IdleArgs) {

    }
}