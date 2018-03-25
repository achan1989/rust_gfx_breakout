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


type Difference = cgmath::Vector2<f32>;

pub enum Collision {
    Yes(Direction, Difference),
    No,
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn vec(&self) -> cgmath::Vector2<f32> {
        use self::cgmath::vec2;

        match *self {
            Direction::Up => vec2(0.0, 1.0),
            Direction::Right => vec2(1.0, 0.0),
            Direction::Down => vec2(0.0, -1.0),
            Direction::Left => vec2(-1.0, 0.0),
        }
    }

    pub fn nearest_from(target: &cgmath::Vector2<f32>) -> Self {
        use self::cgmath::InnerSpace;

        const COMPASS: [Direction; 4] = [
            Direction::Up, Direction::Right, Direction::Down, Direction::Left];
        let mut max = 0.0;
        let mut best = &Direction::Up;
        let target = target.clone().normalize();

        for dir in COMPASS.iter() {
            let dot_product = target.dot(dir.vec());
            if dot_product > max {
                max = dot_product;
                best = dir;
            }
        }
        best.clone()
    }
}
