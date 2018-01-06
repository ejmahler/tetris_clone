
use piston::event_loop::EventSettings;
use piston_window::{Window, PistonWindow, RenderArgs, AfterRenderArgs, UpdateArgs, IdleArgs, Input};

use num_traits::{Zero, One, clamp};

use cgmath::{Decomposed, Vector3, Vector4, Matrix4, Quaternion, ortho};

use ::core::App;

use super::tetris_input::TetrisInput;
use super::render::RenderState;

pub struct TetrisApp<'app> {
    window: &'app mut PistonWindow,
    render_state: RenderState,


    input: TetrisInput,

    projection_matrix: Matrix4<f32>,

    red: f32,
    green: f32,
}

impl<'app> TetrisApp<'app> {
    pub fn new(window: &'app mut PistonWindow) -> Self {
        let window_size = window.window.size();

        let render_state = RenderState::new(window);
        Self {
            window,
            render_state: render_state,
            input: TetrisInput::new(),

            projection_matrix: Self::compute_projection(window_size.width as f32, window_size.height as f32),

            red: 0.0,
            green: 0.0,
        }
    }

    fn compute_projection(width: f32, height: f32) -> Matrix4<f32> {
        if width > height {
            let ratio = width / height;
            ortho(-ratio, ratio, -1.0, 1.0, -1.0, 1.0)
        }
        else {
            let ratio = height / width;
            ortho(-1.0, 1.0, -ratio, ratio, -1.0, 1.0)
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
        let model_view: Decomposed<Vector3<f32>, Quaternion<f32>> = Decomposed {
            scale: 1.0,
            rot: Quaternion::one(),
            disp: Vector3::zero(),
        };
        let model_view_matrix: Matrix4<f32> = model_view.into();

        let color = Vector4::new(self.red, self.green, 1.0, 1.0);

        self.render_state.render_tetris_block(&mut self.window, &(self.projection_matrix * model_view_matrix), &color);
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
            self.red = clamp(self.red - dt, 0.0, 1.0);
        }

        // if the player is holding the right arrow, fade to green
        if self.input.arrow_right.pressed() {
            self.green = clamp(self.green + dt, 0.0, 1.0);
        } else {
            self.green = clamp(self.green - dt, 0.0, 1.0);
        }
    }
    fn idle(&mut self, _: &IdleArgs) {

    }
    fn resize(&mut self, width: u32, height: u32) {
        self.projection_matrix = Self::compute_projection(width as f32, height as f32);

        //rebuild the entire render state. TODO find a way to update each thing's render target without doing this or making the render state mutable
        self.render_state = RenderState::new(&mut self.window);
    }
}