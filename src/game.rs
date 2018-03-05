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

use errors::*;
use renderer;
use resource_manager::ResourceManager;


pub struct Game <F: gfx::traits::FactoryExt<R>, R: gfx::Resources> {
    height: i32,
    width: i32,
    state: GameState,
    projection: cgmath::Matrix4<f32>,
    resources: ResourceManager<F, R>,
    sprite_renderer: renderer::SpriteRenderer<R>,
}

pub enum GameState {
    Active,
    Menu,
    Win,
}

impl<F: gfx::traits::FactoryExt<R> + Clone, R: gfx::Resources> Game<F, R> {
    pub fn new(
        fb_width: i32, fb_height: i32,
        mut factory: F, fb: renderer::RenderTargetView<R>
        )-> Result<Self>
    {
        let mut resources = ResourceManager::new(factory.clone());
        resources.load_shader(
            &"assets/shaders/sprite.vs", &"assets/shaders/sprite.fs", None,
            "sprite".into())?;
        resources.load_texture(
            &"assets/textures/awesomeface.png",
            "face".into())?;

        // left, right, bottom, top, near, far.
        // Note that bottom and top are "backwards", with y increasing down
        // the screen.
        let projection = cgmath::ortho(
            0.0, fb_width as f32,
            fb_height as f32, 0.0,
            -1.0, 1.0);

        let sprite_renderer = renderer::SpriteRenderer::new(
            resources.shader("sprite").unwrap(),
            &projection,
            &mut factory,
            fb.clone())?;

        Ok(Self {
            height: fb_height,
            width: fb_width,
            state: GameState::Active,
            projection,
            resources,
            sprite_renderer,
        })
    }

    pub fn process_input(&mut self, delta_time: f32) {
        // todo
    }

    pub fn update(&mut self, delta_time: f32) {
        // todo
    }

    pub fn render<C: gfx::CommandBuffer<R>>(
        &mut self, encoder: &mut gfx::Encoder<R, C>)
    {
        // For some reason the face sprite is displayed with a white border.
        // I think this is some kind of encoding error in the image, since
        // replacing the image with something else makes the white border go
        // away...
        self.sprite_renderer.draw_sprite(
            self.resources.texture("face").unwrap(),
            cgmath::vec2(200.0, 200.0),
            cgmath::vec2(300.0, 400.0),
            45.0,
            cgmath::vec3(0.0, 1.0, 0.0),
            encoder);
    }
}
