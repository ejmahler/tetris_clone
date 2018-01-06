
use piston::event_loop::EventSettings;
use piston_window::{Window, PistonWindow, RenderArgs, AfterRenderArgs, UpdateArgs, IdleArgs, Input};

use num_traits::{Zero, One};

use cgmath::{Decomposed, Vector3, Matrix4, Quaternion, ortho};
use rand::thread_rng;

use ::core::App;
use ::core::intvector::IntVector2;

use super::tetris_input::TetrisInput;
use super::render::RenderState;
use super::tetris_piece::TetrisPiece;

pub struct TetrisApp<'app> {
    window: &'app mut PistonWindow,
    render_state: RenderState,

    active_piece: TetrisPiece,
    input: TetrisInput,

    projection_matrix: Matrix4<f32>,
}

impl<'app> TetrisApp<'app> {
    pub fn new(window: &'app mut PistonWindow) -> Self {
        let window_size = window.window.size();

        let render_state = RenderState::new(window);
        Self {
            window,
            render_state: render_state,

            active_piece: TetrisPiece::new_random(IntVector2::new(0,0), &mut thread_rng()),
            input: TetrisInput::new(),

            projection_matrix: Self::compute_projection(window_size.width as f32, window_size.height as f32),
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
        self.window.encoder.clear(&self.window.output_color, [0.0, 0.0, 0.0, 1.0]);

        let view_matrix: Matrix4<f32> = Decomposed::<Vector3<f32>, Quaternion<f32>> {
            scale: 0.25,
            rot: Quaternion::one(),
            disp: Vector3::zero(),
        }.into();
        let view_projection_matrix = self.projection_matrix * view_matrix;

        let piece_color = self.active_piece.get_color();

        for cell in &self.active_piece.get_occupied_cells() {
            let model_matrix: Matrix4<f32> = Decomposed::<Vector3<f32>, Quaternion<f32>> {
                scale: 1.0,
                rot: Quaternion::one(),
                disp: Vector3 { x: cell.x as f32, y: cell.y as f32, z: 0.0 },
            }.into();
            let mvp_matrix = view_projection_matrix * model_matrix;

            self.render_state.render_tetris_block(&mut self.window, &mvp_matrix, &piece_color);
        }

        self.window.encoder.flush(&mut self.window.device);
    }
    fn after_render(&mut self, _: &AfterRenderArgs) {

    }


    fn update(&mut self, args: &UpdateArgs, input_events: &[Input]) {
        let dt = args.dt as f32;

        // update our input axes
        self.input.update(dt, input_events);
        
        // if the player pressed the left arrow this frame only, set red to 1
        if self.input.arrow_left.pressed_this_frame() {
            self.active_piece = self.active_piece.rotated_left();
        }
        if self.input.arrow_right.pressed_this_frame() {
            self.active_piece = self.active_piece.rotated_right();
        }
        if self.input.arrow_up.pressed_this_frame() {
            self.active_piece = TetrisPiece::new_random(IntVector2::new(0,0), &mut thread_rng());
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