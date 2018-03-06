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

extern crate cgmath;
extern crate gfx;

use texture;


pub struct GameObject<R: gfx::Resources> {
    pub position: cgmath::Vector2<f32>,
    pub size: cgmath::Vector2<f32>,
    pub velocity: cgmath::Vector2<f32>,
    pub colour: cgmath::Vector3<f32>,
    pub rotation: f32,
    pub is_solid: bool,
    pub is_destroyed: bool,
    pub sprite: texture::Texture2D<R>,
}

impl <R: gfx::Resources> GameObject<R> {
    pub fn new(
        position: cgmath::Vector2<f32>, size: cgmath::Vector2<f32>,
        sprite: &texture::Texture2D<R>,
        colour: cgmath::Vector3<f32>)
    -> Self
    {
        use self::cgmath::vec2;

        Self {
            position,
            size,
            velocity: vec2(0.0, 0.0),
            colour,
            rotation: 0.0,
            is_solid: false,
            is_destroyed: false,
            sprite: sprite.clone(),
        }
    }
}
