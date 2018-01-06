use piston_window::PistonWindow;
use gfx_device_gl::Resources;
use cgmath::{Vector4, Matrix4};

mod tetris_block;

pub struct RenderState {
    tetris_block_data: tetris_block::TetrisBlock<Resources>,
}

impl RenderState {
    pub fn new(window: &mut PistonWindow) -> Self {
        Self {
            tetris_block_data: tetris_block::TetrisBlock::new(&mut window.factory, &window.output_color),
        }
    }

    pub fn render_tetris_block(&self, window: &mut PistonWindow, transform: &Matrix4<f32>, tint_color: &Vector4<f32>) {
        window.encoder.clear(&window.output_color, [0.0, 0.0, 0.0, 1.0]);
        self.tetris_block_data.render(&mut window.encoder, transform, tint_color);
        window.encoder.flush(&mut window.device);
    }
}