
use gfx;
use gfx::format::Srgba8;
use gfx::pso::PipelineState;
use gfx::Slice;
use gfx::traits::FactoryExt;
use gfx::handle::RenderTargetView;
use cgmath::{Vector4, Transform3, Matrix4};

gfx_defines!{
    vertex Vertex {
        position: [f32; 3] = "in_position",
        uv0: [f32; 2] = "in_uv0",
    }

    constant Globals {
        transform: [[f32; 4];4] = "u_transform",
        tint_color: [f32; 4] = "u_tintColor",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        globals: gfx::ConstantBuffer<Globals> = "Globals",
        out: gfx::RenderTarget<Srgba8> = "Target0",
    }
}

const QUAD_VERTS : [Vertex; 4] = [
    Vertex { position: [-0.5, -0.5, 0.0], uv0 : [0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], uv0 : [1.0, 0.0] },
    Vertex { position: [-0.5,  0.5, 0.0], uv0 : [0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, 0.0], uv0 : [1.0, 1.0] },
];
const QUAD_INDICES: [u16; 6] = [
    0, 1, 2, // first triangle
    2, 1, 3, // second triangle
];


pub struct TetrisBlock<R: gfx::Resources> {
    vbuf_slice: Slice<R>,
    pso: PipelineState<R, pipe::Meta>,
    pso_data: pipe::Data<R>,
}

impl<R: gfx::Resources> TetrisBlock<R> {
    pub fn new(factory: &mut impl FactoryExt<R>, target: &RenderTargetView<R, Srgba8>) -> Self {
        let index_slice: &[u16] = &QUAD_INDICES;
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&QUAD_VERTS, index_slice);

        Self {
            vbuf_slice: slice,
            pso: factory.create_pipeline_simple(
                include_bytes!("tetris_block.glslv"),
                include_bytes!("tetris_block.glslf"),
                pipe::new()
            ).unwrap(),
            pso_data: pipe::Data {
                vbuf: vertex_buffer,
                globals: factory.create_constant_buffer(1),
                out: target.clone(),
            },
        }
    }

    pub fn render(&self, encoder: &mut gfx::Encoder<R, impl gfx::CommandBuffer<R>>, transform: &impl Transform3<f32> + Copy, tint_color: &Vector4<f32>) {
        let matrix: Matrix4<f32> = (*transform).into();

        let globals = Globals {
            transform: matrix.into(), 
            tint_color: tint_color.clone().into(),
        };

        encoder.update_constant_buffer(&self.pso_data.globals, &globals);
        encoder.draw(&self.vbuf_slice, &self.pso, &self.pso_data);
    }
}