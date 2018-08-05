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

use gfx;
use image;

use errors::*;


pub type TextureFormat = gfx::format::Rgba8;
pub type TextureSurface = gfx::format::R8_G8_B8_A8;
pub type SrvType = [f32; 4];

// Represents a 2D RGBA texture.
// Unlike the original code:
//   * Textures always have an alpha channel.
//   * Texture wrapping and filter mode options have been removed (they were
//     always the same anyway).
#[derive(Clone)]
pub struct Texture2D<R>
    where R: gfx::Resources
{
    pub surface: gfx::handle::Texture<R, TextureSurface>,
    pub view: gfx::handle::ShaderResourceView<R, SrvType>,
}

impl<R> Texture2D<R>
    where R: gfx::Resources
{
    pub fn new<F: gfx::traits::FactoryExt<R>>(
        img: image::RgbaImage, factory: &mut F)
        -> Result<Self>
    {
        let (width, height) = img.dimensions();
        let kind = gfx::texture::Kind::D2(
            width as u16, height as u16, gfx::texture::AaMode::Single);
        let (tex, srv) =
            factory.create_texture_immutable_u8::<TextureFormat>(
                kind, gfx::texture::Mipmap::Provided, &[&img])?;
        Ok(Self {
            surface: tex,
            view: srv,
        })
    }
}
