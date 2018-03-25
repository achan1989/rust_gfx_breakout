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

use std::collections::HashMap;

extern crate cgmath;
extern crate gfx;
extern crate glfw;

use collision::Collision;
use errors::*;
use game_level::GameLevel;
use game_object;
use game_object::{BallObject, GameObject};
use renderer;
use resource_manager::ResourceManager;


// Use the colour of the sprite as-in.
macro_rules! base_colour {
    () => { cgmath::vec3(1.0, 1.0, 1.0) }
}


pub struct Game <F: gfx::traits::FactoryExt<R>, R: gfx::Resources> {
    height: i32,
    width: i32,
    state: GameState,
    projection: cgmath::Matrix4<f32>,
    resources: ResourceManager<F, R>,
    sprite_renderer: renderer::SpriteRenderer<R>,
    levels: Vec<GameLevel<R>>,
    level: usize,
    player: GameObject<R>,
    ball: BallObject<R>,
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
        resources.load_texture(
            &"assets/textures/paddle.png",
            "paddle".into())?;

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

        let player_size = game_object::initial_player_size();
        let player_pos = cgmath::vec2(
            (fb_width as f32 / 2.0) - (player_size.x / 2.0),
            fb_height as f32 - player_size.y);
        let player = GameObject::new(
            player_pos, player_size,
            resources.texture("paddle").unwrap(),
            base_colour!());

        let ball_radius = <BallObject<R>>::initial_radius();
        let initial_ball_velocity = <BallObject<R>>::initial_velocity();
        let ball_pos = player_pos + cgmath::vec2(
            player_size.x / 2.0 - ball_radius,
            -ball_radius * 2.0);
        let ball = BallObject::new(
            ball_pos, ball_radius, initial_ball_velocity,
            resources.texture("face").unwrap(),
            base_colour!());

        Ok(Self {
            height: fb_height,
            width: fb_width,
            state: GameState::Active,
            projection,
            resources,
            sprite_renderer,
            levels,
            level: 1,
            player,
            ball,
        })
    }

    pub fn process_input(
        &mut self, delta_time: f32, keys: &HashMap<glfw::Key, bool>)
    {
        if let GameState::Active = self.state {
            const PLAYER_VELOCITY: f32 = 500.0;
            let velocity = PLAYER_VELOCITY * delta_time;

            // Movement.
            let old_x = self.player.position.x;
            if *keys.get(&glfw::Key::A).unwrap_or(&false) {
                self.player.position.x -= velocity;
                if self.player.position.x < 0.0 {
                    self.player.position.x = 0.0;
                }
            }
            if *keys.get(&glfw::Key::D).unwrap_or(&false) {
                self.player.position.x += velocity;
                if self.player.position.x + self.player.size.x > self.width as f32 {
                    self.player.position.x = self.width as f32 - self.player.size.x;
                }
            }

            if self.ball.is_stuck() {
                let dx = self.player.position.x - old_x;
                self.ball.move_with_paddle(dx);
            }

            // Release ball.
            if *keys.get(&glfw::Key::Space).unwrap_or(&false) {
                self.ball.release();
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.ball.do_move(delta_time, self.width as f32);
        self.do_collisions();
        if self.ball.is_below(self.height as f32) {
            self.reset_level();
            self.reset_player();
        }
    }

    fn reset_level(&mut self) {
        self.levels[self.level - 1].reset();
    }

    fn reset_player(&mut self) {
        use self::cgmath::vec2;

        let player_size = game_object::initial_player_size();
        self.player.size = player_size;
        self.player.position = vec2(
            (self.width as f32 / 2.0) - (player_size.x / 2.0),
            self.height as f32 - player_size.y);

        let ball_radius = <BallObject<R>>::initial_radius();
        self.ball.reset(
            self.player.position + vec2(
                (player_size.x / 2.0) - ball_radius, -(ball_radius * 2.0)),
            <BallObject<R>>::initial_velocity());
    }

    fn do_collisions(&mut self) {
        for brick in self.levels[self.level - 1].bricks_iter_mut() {
            if !brick.is_destroyed {
                if let Collision::Yes(direction, penetration) =
                       self.ball.check_collision(brick)
               {
                    if !brick.is_solid {
                        brick.is_destroyed = true;
                    }
                    self.ball.rebound_brick(direction, penetration);
                }
            }
        }

        if !self.ball.is_stuck() {
            if let Collision::Yes(_, _) =
                   self.ball.check_collision(&self.player)
            {
                self.ball.rebound_paddle(&self.player);
            }
        }
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
                base_colour!(),
                encoder);

            self.levels[self.level - 1].draw(
                &mut self.sprite_renderer, encoder);

            self.player.draw(&mut self.sprite_renderer, encoder);
            self.ball.draw(&mut self.sprite_renderer, encoder);
        }
    }
}
