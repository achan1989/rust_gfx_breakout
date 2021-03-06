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
use num_traits;

use collision;
use renderer;
use texture;


pub fn initial_player_size() -> cgmath::Vector2<f32> {
    cgmath::vec2(100.0, 20.0)
}


#[derive(Clone)]
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

    pub fn draw<C: gfx::CommandBuffer<R>>(
        &self,
        renderer: &mut renderer::SpriteRenderer<R>,
        encoder: &mut gfx::Encoder<R, C>)
    {
        renderer.draw_sprite(
            &self.sprite,
            self.position,
            self.size,
            self.rotation,
            self.colour,
            encoder);
    }
}


pub struct BallObject<R: gfx::Resources> {
    obj: GameObject<R>,
    radius: f32,
    stuck: bool,
}

impl <R: gfx::Resources> BallObject<R> {
    pub fn initial_velocity() -> cgmath::Vector2<f32> {
        cgmath::vec2(100.0, -350.0)
    }

    pub fn initial_radius() -> f32 {
        12.5
    }

    pub fn new(
        position: cgmath::Vector2<f32>, radius: f32,
        velocity: cgmath::Vector2<f32>,
        sprite: &texture::Texture2D<R>,
        colour: cgmath::Vector3<f32>)
    -> Self
    {
        use self::cgmath::vec2;

        let mut obj = GameObject::new(
            position, vec2(radius * 2.0, radius * 2.0),
            sprite, colour);
        obj.velocity = velocity;
        Self {
            obj,
            radius,
            stuck: true,
        }
    }

    pub fn reset(
        &mut self,
        position: cgmath::Vector2<f32>,
        velocity: cgmath::Vector2<f32>)
    {
        self.obj.position = position;
        self.obj.velocity = velocity;
        self.stuck = true;
    }

    pub fn do_move(&mut self, delta_time: f32, window_width: f32) {
        if !self.stuck {
            self.obj.position += self.obj.velocity * delta_time;
            if self.obj.position.x <= 0.0 {
                self.obj.velocity.x = -self.obj.velocity.x;
                self.obj.position.x = 0.0;
            }
            else if self.obj.position.x + self.obj.size.x >= window_width {
                self.obj.velocity.x = -self.obj.velocity.x;
                self.obj.position.x = window_width - self.obj.size.x;
            }
            if self.obj.position.y <= 0.0 {
                self.obj.velocity.y = -self.obj.velocity.y;
                self.obj.position.y = 0.0;
            }
        }
    }

    pub fn is_stuck(&self) -> bool {
        self.stuck
    }

    pub fn release(&mut self) {
        self.stuck = false;
    }

    pub fn is_below(&self, y: f32) -> bool {
        self.obj.position.y > y
    }

    pub fn move_with_paddle(&mut self, dx: f32) {
        self.obj.position.x += dx;
    }

    pub fn draw<C: gfx::CommandBuffer<R>>(
        &self,
        renderer: &mut renderer::SpriteRenderer<R>,
        encoder: &mut gfx::Encoder<R, C>)
    {
        self.obj.draw(renderer, encoder);
    }

    pub fn check_collision(&self, other: &GameObject<R>) -> collision::Collision
    {
        use self::cgmath::{ElementWise, InnerSpace};
        use self::cgmath::vec2;
        use self::collision::{Collision, Direction};
        use self::num_traits::clamp;

        let center = self.obj.position.add_element_wise(self.radius);
        // AABB info.
        let aabb_half_extents = other.size / 2.0;
        let aabb_center = other.position + aabb_half_extents;
        // Get difference vector between both centers.
        let diff1 = center - aabb_center;
        let clamped = vec2(
            clamp(diff1.x, -aabb_half_extents.x, aabb_half_extents.x),
            clamp(diff1.y, -aabb_half_extents.y, aabb_half_extents.y));
        // Add clamped value to AABB_center and we get the value of box closest
        // to circle.
        let closest = aabb_center + clamped;
        // Retrieve vector between center circle and closest point AABB and check
        // if length <= radius.
        let diff2 = closest - center;
        if diff2.magnitude() <= self.radius {
            let direction = Direction::nearest_from(&diff2);
            Collision::Yes(direction, diff2)
        }
        else {
            Collision::No
        }
    }

    pub fn rebound_brick(
        &mut self,
        direction: collision::Direction,
        difference: cgmath::Vector2<f32>)
    {
        use self::collision::Direction;

        match direction {
            Direction::Left | Direction::Right => {
                self.obj.velocity.x = -self.obj.velocity.x;
                let penetration = self.radius - difference.x.abs();
                if let Direction::Left = direction {
                    self.obj.position.x += penetration;
                } else {
                    self.obj.position.x -= penetration;
                }
            },
            Direction::Up | Direction::Down => {
                self.obj.velocity.y = -self.obj.velocity.y;
                let penetration = self.radius - difference.y.abs();
                if let Direction::Up = direction {
                    self.obj.position.y -= penetration;
                } else {
                    self.obj.position.y += penetration;
                }
            }
        }
    }

    pub fn rebound_paddle(&mut self, paddle: &GameObject<R>) {
        use self::cgmath::InnerSpace;

        // New ball velocity depends on distance from the center of the
        // paddle.
        let center = paddle.position.x + (paddle.size.x / 2.0);
        let distance = (self.obj.position.x + self.radius) - center;
        let percentage = distance / (paddle.size.x / 2.0);

        const STRENGTH: f32 = 2.0;
        // Overall speed is preserved.
        let speed = self.obj.velocity.magnitude();
        self.obj.velocity.x = Self::initial_velocity().x * percentage * STRENGTH;
        self.obj.velocity.y = -1.0 * self.obj.velocity.y.abs();
        self.obj.velocity = self.obj.velocity.normalize() * speed;
    }
}
