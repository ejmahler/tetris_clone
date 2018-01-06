use std::path::PathBuf;

use gfx;
use gfx::format::Srgba8;
use gfx::pso::PipelineState;
use gfx::Slice;
use gfx::traits::FactoryExt;
use gfx::handle::RenderTargetView;
use cgmath::{Vector4, Matrix4};
use piston_window::{Texture, TextureSettings, Flip, Filter};

gfx_defines!{
    vertex Vertex {
        position: [f32; 3] = "in_position",
        uv0: [f32; 2] = "in_uv0",
    }

    constant Transients {
        transform: [[f32; 4];4] = "u_transform",
        tint_color: [f32; 4] = "u_tintColor",
    }

    /*constant Constants {
        albedoMap: ,
    }*/

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transients: gfx::ConstantBuffer<Transients> = "Transients",
        texture_albedo: gfx::TextureSampler<[f32; 4]> = "t_albedoMap",
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

    #[allow(unused)]
    albedo_data: Texture<R>,
}

impl<R: gfx::Resources> TetrisBlock<R> {
    pub fn new(factory: &mut impl FactoryExt<R>, target: &RenderTargetView<R, Srgba8>) -> Self {

        //set up the VBO
        let index_slice: &[u16] = &QUAD_INDICES;
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&QUAD_VERTS, index_slice);

        //set up the textures
        let texture_path: PathBuf = ["resources","textures","tetris_square_albedo.png"].iter().collect();
        println!("{:?}", texture_path);
        let texture_settings = TextureSettings::new().mag(Filter::Nearest);

        let albedo_tex = Texture::from_path(factory, texture_path, Flip::None, &texture_settings).unwrap();

        // combine everything
        Self {
            vbuf_slice: slice,
            pso: factory.create_pipeline_simple(
                include_bytes!("tetris_block.glslv"),
                include_bytes!("tetris_block.glslf"),
                pipe::new()
            ).unwrap(),
            pso_data: pipe::Data {
                vbuf: vertex_buffer,
                transients: factory.create_constant_buffer(1),
                texture_albedo: (albedo_tex.view.clone(), albedo_tex.sampler.clone()),
                out: target.clone(),
            },
            albedo_data: albedo_tex
        }
    }

    pub fn render(&self, encoder: &mut gfx::Encoder<R, impl gfx::CommandBuffer<R>>, transform: &Matrix4<f32>, tint_color: &Vector4<f32>) {
        let transient_data = Transients {
            transform: (*transform).into(), 
            tint_color: (*tint_color).into(),
        };

        encoder.update_constant_buffer(&self.pso_data.transients, &transient_data);
        encoder.draw(&self.vbuf_slice, &self.pso, &self.pso_data);
    }
}