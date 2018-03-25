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

use std::fs;
use std::io;
use std::path::Path;

extern crate cgmath;
extern crate gfx;

use errors::*;
use game_object::GameObject;
use renderer;
use resource_manager::ResourceManager;


enum TileKind {
    Empty,
    Solid,
    Blue,
    Green,
    Tan,
    Orange,
    White,
}

impl ::std::convert::From<u8> for TileKind {
    fn from(n: u8) -> TileKind {
        match n {
            0 => TileKind::Empty,
            1 => TileKind::Solid,
            2 => TileKind::Blue,
            3 => TileKind::Green,
            4 => TileKind::Tan,
            5 => TileKind::Orange,
            _ => TileKind::White,
        }
    }
}

impl TileKind {
    fn try_from(s: &str) -> Result<TileKind> {
        let n: u8 = s.parse()?;
        Ok(TileKind::from(n))
    }

    fn colour(&self) -> cgmath::Vector3<f32> {
        use self::TileKind::*;
        use self::cgmath::vec3;

        match *self {
            Empty => vec3(0.0, 0.0, 0.0),
            Solid => vec3(0.8, 0.8, 0.7),
            Blue => vec3(0.2, 0.6, 1.0),
            Green => vec3(0.0, 0.7, 0.0),
            Tan => vec3(0.8, 0.8, 0.4),
            Orange => vec3(1.0, 0.5, 0.0),
            White => vec3(1.0, 1.0, 1.0),
        }
    }

    fn is_solid(&self) -> bool {
        match *self {
            TileKind::Solid => true,
            _ => false,
        }
    }

    fn texture_name(&self) -> &str {
        match *self {
            TileKind::Empty => panic!("empty tiles have no texture"),
            TileKind::Solid => "block_solid",
            _ => "block",
        }
    }
}

pub struct GameLevel<R: gfx::Resources> {
    bricks: Vec<GameObject<R>>,
}

impl <R: gfx::Resources> GameLevel<R> {
    pub fn new<P: AsRef<Path>, F: gfx::traits::FactoryExt<R>>(
        path: &P, level_width: u32, level_height: u32,
        resources: &ResourceManager<F, R>)
    -> Result<Self>
    {
        let tile_data = Self::read_tile_data(path.as_ref())?;

        let height = tile_data.len();
        let width = tile_data[0].len();
        let unit_width = level_width as f32 / width as f32;
        let unit_height = level_height as f32 / height as f32;
        if !tile_data.iter().all(|row| row.len() == width) {
            bail!("expected all rows to be {} wide", width);
        }

        let mut bricks = Vec::with_capacity(150);
        for (y, tile_row) in tile_data.iter().enumerate() {
            for (x, tile) in tile_row.iter().enumerate() {
                use self::cgmath::vec2;

                if let &TileKind::Empty = tile {
                    continue;
                }

                let pos = vec2(unit_width * x as f32, unit_height * y as f32);
                let size = vec2(unit_width, unit_height);
                let colour = tile.colour();
                let sprite = {
                    let texture_name = tile.texture_name();
                    let texture = resources.texture(texture_name);
                    if let None = texture {
                        bail!("no texture resource for {}", texture_name);
                    }
                    texture.unwrap()
                };

                let mut obj = GameObject::new(pos, size, sprite, colour);
                obj.is_solid = tile.is_solid();
                bricks.push(obj);
            }
        }

        Ok(Self{
            bricks,
        })
    }

    pub fn is_completed(&self) -> bool {
        self.bricks.iter().all(|b| b.is_solid || b.is_destroyed )
    }

    pub fn draw<C: gfx::CommandBuffer<R>>(
        &self,
        renderer: &mut renderer::SpriteRenderer<R>,
        encoder: &mut gfx::Encoder<R, C>)
    {
        for tile in self.bricks.iter().filter(|t| !t.is_destroyed) {
            tile.draw(renderer, encoder);
        }
    }

    pub fn bricks_iter_mut(&mut self) -> ::std::slice::IterMut<GameObject<R>> {
        self.bricks.iter_mut()
    }

    fn read_tile_data(path: &Path) -> Result<Vec<Vec<TileKind>>> {
        use std::io::BufRead;
        let mut tile_data = Vec::with_capacity(10);
        let reader = io::BufReader::new(fs::File::open(path)?);

        for line in reader.lines() {
            let line = line?;
            let tiles: Result<Vec<_>> =
                line
                .split_whitespace()
                .map(|s| TileKind::try_from(s) )
                .collect();
            tile_data.push(tiles?);
        }

        if tile_data.len() == 0 {
            bail!("no level data in {:?}", path);
        }
        Ok(tile_data)
    }
}
