// This code is part of Breakout.
//
// Breakout is free software: you can redistribute it and/or modify it under
// the terms of the CC BY 4.0 license as published by Creative Commons, either
// version 4 of the License, or (at your option) any later version.
//
// https://creativecommons.org/licenses/by/4.0/legalcode
//
// The original code is copyright Joey de Vries
// (https://twitter.com/JoeyDeVriez) and can be found at
// https://learnopengl.com/In-Practice/2D-Game/Breakout
//
// The original code was modified by Adrian Chan in order to port it to Rust.

use cgmath;
use gfx;
use image;

use errors::*;
use texture;


// Could these be generic, or does the renderer have to make some assumptions
// about the colour format?  I've made that assumption anyway, and have
// accepted tighter coupling with the backend/windowing system.
// These types are also duplicated in main.rs :(
pub type ColourFormat = gfx::format::Rgba8;
pub type RenderTargetView<R: gfx::Resources> =
    gfx::handle::RenderTargetView<R, ColourFormat>;

pub struct SpriteRenderer <R: gfx::Resources> {
    // We can use the Bundle struct to slightly simplify the storage of the PSO
    // and associated data, since the set of vertices we draw never changes
    // between frames.
    pso_bundle: gfx::pso::bundle::Bundle<R, pipe::Data<R>>,
}

impl <R: gfx::Resources> SpriteRenderer <R> {
    pub fn new<F: gfx::traits::FactoryExt<R>>(
        shader: &gfx::handle::Program<R>, projection: &cgmath::Matrix4<f32>,
        factory: &mut F, fb: RenderTargetView<R>
        ) -> Result<Self>
    {
        let pso = factory.create_pipeline_from_program(
            shader,
            gfx::Primitive::TriangleList,
            gfx::state::Rasterizer::new_fill().with_cull_back(),
            pipe::new())
            // For some reason this is the only create_pipeline_* variant that
            // returns errors with str instead of String. Convert to String
            // so that it plays nice with error_chain.
            .map_err(|e| gfx::PipelineStateError::<String>::from(e))?;

        // This defines a sprite so that (0,0) is the top-left corner, and
        // (1,1) is the bottom-right corner. This is the normal layout for
        // images -- using (0,0) for the bottom-left leads to misery...
        const VERTICES: &[Vertex] = &[
            Vertex { pos: [0.0, 0.0], uv: [0.0, 0.0] },  // top left
            Vertex { pos: [1.0, 0.0], uv: [1.0, 0.0] },  // top right
            Vertex { pos: [0.0, 1.0], uv: [0.0, 1.0] },  // bottom left
            Vertex { pos: [1.0, 1.0], uv: [1.0, 1.0] },  // bottom right
        ];
        const INDICES: &[u16] = &[
            0, 3, 1,
            0, 2, 3,
        ];
        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(VERTICES, INDICES);

        // The sampler must be created in a valid state, so we need to give it
        // some sort of texture now.
        // I was lazy and created a dummy 1x1 texture here. Once we render a
        // sprite with a proper texture, the dummy texture will be destroyed
        // automatically.
        // There are probably better ways to do this.
        let sampler = factory.create_sampler(
            gfx::texture::SamplerInfo::new(
                gfx::texture::FilterMethod::Bilinear,
                gfx::texture::WrapMode::Tile));
        let default_texture = {
            let pixel = image::Rgba { data: [255, 0, 255, 255] };
            let img = image::RgbaImage::from_pixel(1, 1, pixel);
            texture::Texture2D::new(img, factory)?
        };

        let data = pipe::Data {
            vertex_buffer,
            sprite_sampler: (default_texture.view, sampler),
            locals: factory.create_constant_buffer(1),
            projection: projection.clone().into(),
            out: fb,
        };

        let pso_bundle = gfx::pso::bundle::Bundle::new(
            slice, pso, data);

        Ok(Self {
            pso_bundle,
        })
    }

    pub fn draw_sprite<C: gfx::CommandBuffer<R>>(
        &mut self,
        texture: &texture::Texture2D<R>,
        position: cgmath::Vector2<f32>,
        size: cgmath::Vector2<f32>,
        rotation: f32,
        colour: cgmath::Vector3<f32>,
        encoder: &mut gfx::Encoder<R, C>)
    {
        use self::cgmath::{Deg, Matrix4};

        // Making the individual matrices like this makes it easy to follow
        // what is going on, but is probably not ideal for memory.
        // Or is rustc smart enough to optimise away some of this?

        // Sprite origin is top-left corner (0,0).
        // Main translation for position.
        let trans = Matrix4::from_translation(position.extend(0.0));

        // Move the origin of rotation to the center of the quad, rotate, then
        // restore the origin.
        let rot = {
            let trans_for_rot = Matrix4::from_translation((size / 2.0).extend(0.0));
            let rot = Matrix4::from_angle_z(Deg(rotation));
            let trans_back_rot = Matrix4::from_translation((size / -2.0).extend(0.0));
            trans_for_rot * rot * trans_back_rot
        };

        let scale = Matrix4::from_nonuniform_scale(size.x, size.y, 1.0);

        // Combine -- scale then rotate then position.
        let model = trans * rot * scale;

        let locals = Locals {
            colour: colour.extend(1.0).into(),
            model: model.into(),
        };

        encoder.update_constant_buffer(&self.pso_bundle.data.locals, &locals);
        self.pso_bundle.data.sprite_sampler.0 = texture.view.clone();
        self.pso_bundle.encode(encoder);
    }
}


gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "position",
        uv: [f32; 2] = "texCoords",
    }

    // Some deviation from the original code, just for the sake of
    // experimenting. The colour and model are now stored in a UBO (assuming
    // OpenGL backend).
    // Could we compute the colour and model once per game object and store
    // the data, instead of re-computing it for every sprite on every frame?
    constant Locals {
        colour: [f32; 4] = "spriteColour",
        model: [[f32; 4]; 4] = "model",
    }

    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        sprite_sampler: gfx::TextureSampler<[f32; 4]> = "image",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        projection: gfx::Global<[[f32; 4]; 4]> = "projection",
        // Use BlendTarget for any transparency that is more complicated than
        // on/off.
        out: gfx::RenderTarget<ColourFormat> = "target",
    }
}
