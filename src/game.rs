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
use game_level::GameLevel;
use renderer;
use resource_manager::ResourceManager;


pub struct Game <F: gfx::traits::FactoryExt<R>, R: gfx::Resources> {
    height: i32,
    width: i32,
    state: GameState,
    projection: cgmath::Matrix4<f32>,
    resources: ResourceManager<F, R>,
    sprite_renderer: renderer::SpriteRenderer<R>,
    levels: Vec<GameLevel<R>>,
    level: usize,
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
        // Textures.
        resources.load_texture(
            &"assets/textures/awesomeface.png",
            "face".into())?;
        resources.load_texture(
            &"assets/textures/background.jpg",
            "background".into())?;
        resources.load_texture(
            &"assets/textures/block.png",
            "block".into())?;
        resources.load_texture(
            &"assets/textures/block_solid.png",
            "block_solid".into())?;

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

        let level_data = [
            "assets/levels/one.lvl",
            "assets/levels/two.lvl",
            "assets/levels/three.lvl",
            "assets/levels/four.lvl",];
        let mut levels = Vec::with_capacity(level_data.len());
        for level in level_data.iter() {
            let lvl = GameLevel::new(
                level, fb_width as u32, (fb_height / 2) as u32, &resources)?;
            levels.push(lvl);
        }

        Ok(Self {
            height: fb_height,
            width: fb_width,
            state: GameState::Active,
            projection,
            resources,
            sprite_renderer,
            levels,
            level: 1,
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
        if let GameState::Active = self.state
        {
            self.sprite_renderer.draw_sprite(
                self.resources.texture("background").unwrap(),
                cgmath::vec2(0.0, 0.0),
                cgmath::vec2(self.width as f32, self.height as f32),
                0.0,
                cgmath::vec3(1.0, 1.0, 1.0),
                encoder);

            self.levels[self.level - 1].draw(
                &mut self.sprite_renderer, encoder);
        }
    }
}
