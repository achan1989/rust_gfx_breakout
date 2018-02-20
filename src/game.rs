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

pub struct Game {
    height: i32,
    width: i32,
    state: GameState,
}

pub enum GameState {
    Active,
    Menu,
    Win,
}

impl Game {
    pub fn new(fb_width: i32, fb_height: i32) -> Self {
        Self {
            height: fb_height,
            width: fb_width,
            state: GameState::Active,
        }
    }

    pub fn process_input(&mut self, delta_time: f32) {
        // todo
    }

    pub fn update(&mut self, delta_time: f32) {
        // todo
    }

    pub fn render(&mut self) {
        // todo
    }
}
