
use piston::event_loop::EventSettings;
use piston_window::{Window, PistonWindow, RenderArgs, AfterRenderArgs, UpdateArgs, IdleArgs, Input};

use num_traits::{Zero, One};

use cgmath::{Decomposed, Vector3, Matrix4, Quaternion, ortho};
use rand::thread_rng;

use ::engine::App;
use ::engine::intvector::IntVector2;

use super::tetris_input::TetrisInput;
use super::render::RenderState;
use super::tetromino::Tetromino;
use super::playfield::Playfield;

enum TetrominoState {
    Spawning(f32),
    Active(Tetromino, f32),
    GameOver,
}

pub struct TetrisApp<'app> {
    window: &'app mut PistonWindow,
    render_state: RenderState,

    playfield: Playfield,
    state: TetrominoState,
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

            playfield: Playfield::new_empty(),
            state: TetrominoState::Spawning(0.0),
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
            scale: 0.05,
            rot: Quaternion::one(),
            disp: Vector3::zero(),
        }.into();
        let view_projection_matrix = self.projection_matrix * view_matrix;

        // render any orphan blocks
        for (cell, orphan) in self.playfield.iter_orphans() {
            let model_matrix: Matrix4<f32> = Decomposed::<Vector3<f32>, Quaternion<f32>> {
                scale: 1.0,
                rot: Quaternion::one(),
                disp: Vector3 { x: cell.x as f32, y: cell.y as f32, z: 0.0 },
            }.into();
            let mvp_matrix = view_projection_matrix * model_matrix;

            self.render_state.render_tetris_block(&mut self.window, &mvp_matrix, &orphan.color);
        }

        // render the active piece, if present
        if let &TetrominoState::Active(ref active_tetromino, _) = &self.state {
            let piece_color = active_tetromino.get_color();

            for cell in &active_tetromino.get_occupied_cells() {
                let model_matrix: Matrix4<f32> = Decomposed::<Vector3<f32>, Quaternion<f32>> {
                    scale: 1.0,
                    rot: Quaternion::one(),
                    disp: Vector3 { x: cell.x as f32, y: cell.y as f32, z: 0.0 },
                }.into();
                let mvp_matrix = view_projection_matrix * model_matrix;

                self.render_state.render_tetris_block(&mut self.window, &mvp_matrix, &piece_color);
            }
        }

        self.window.encoder.flush(&mut self.window.device);
    }
    fn after_render(&mut self, _: &AfterRenderArgs) {

    }


    fn update(&mut self, args: &UpdateArgs, input_events: &[Input]) {
        let dt = args.dt as f32;

        // update our input axes
        self.input.update(dt, input_events);

        // If we're waiting to spawn a new tetromino, update the cooldown
        if let TetrominoState::Spawning(mut cooldown) = self.state {
            cooldown -= dt;
            if cooldown < 0.0 {
                let new_tetromino = Tetromino::new_random(self.playfield.spawn_location(), &mut thread_rng());
                if !self.playfield.is_valid_placement(&new_tetromino) {
                    self.state = TetrominoState::GameOver;
                }
                else {
                    self.state = TetrominoState::Active(new_tetromino, 0.0)
                }
            }
            else {
                self.state = TetrominoState::Spawning(cooldown)
            }
        }

        // If we have an active tetromino, update its position from gravity
        if let TetrominoState::Active(active_tetromino, mut drop_cooldown) = self.state {
            drop_cooldown -= dt;

            // If we've hit the drop cooldown, move this piece down by 1 row. If we can't, lock it in place.
            if drop_cooldown < 0.0 {
                let moved_tetromino = active_tetromino.moved(IntVector2::new(0, -1));
                if self.playfield.is_valid_placement(&moved_tetromino) {
                    self.state = TetrominoState::Active(moved_tetromino, 1.0)
                }
                else {
                    self.playfield.lock_tetromino(&active_tetromino);
                    self.state = TetrominoState::Spawning(1.0)
                }
            }
            else {
                self.state = TetrominoState::Active(active_tetromino, drop_cooldown)
            }
        }

        // If we STILL have an active tetromino after dropping, handle player input
        if let &mut TetrominoState::Active(ref mut active_tetromino, _) = &mut self.state {

            // try to move the tetromino left
            if self.input.arrow_left.pressed_this_frame() {
                let updated_tetromino = active_tetromino.moved(IntVector2::new(-1, 0));
                if self.playfield.is_valid_placement(&updated_tetromino) {
                    *active_tetromino = updated_tetromino;
                }
            }

            // try to move the tetromino right
            if self.input.arrow_right.pressed_this_frame() {
                let updated_tetromino = active_tetromino.moved(IntVector2::new(1, 0));
                if self.playfield.is_valid_placement(&updated_tetromino) {
                    *active_tetromino = updated_tetromino;
                }
            }

            // try to rotate the tetromino left
            if self.input.key_a.pressed_this_frame() {
                let updated_tetromino = active_tetromino.rotated_left();
                if self.playfield.is_valid_placement(&updated_tetromino) {
                    *active_tetromino = updated_tetromino;
                }
            }

            // try to rotate the tetromino right
            if self.input.key_d.pressed_this_frame() {
                let updated_tetromino = active_tetromino.rotated_right();
                if self.playfield.is_valid_placement(&updated_tetromino) {
                    *active_tetromino = updated_tetromino;
                }
            }
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