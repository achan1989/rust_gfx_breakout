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

// This resource manager is basically the same as the original version,
// although we create our resources through an instance of a gfx Factory,
// instead of mutating global state as per raw OpenGL.
// The Factory instance can be cloned cheaply and stored/used in multiple
// places -- the only data it contains is a referenced counted handle to the
// underlying context.
//
// The original Shader utility class is completely gone from our code, since
// gfx already provides that abstraction and functionality.
// Note that the original "fragment shader" has become a "pixel shader" under
// the new gfx system.

use std::collections::HashMap;
use std::path::Path;

extern crate gfx;
use gfx::handle;

use errors::*;


pub struct ResourceManager<F: gfx::traits::FactoryExt<R>, R: gfx::Resources> {
    factory: F,
    shaders: HashMap<String, handle::Program<R>>,
}

impl<F: gfx::traits::FactoryExt<R>, R: gfx::Resources> ResourceManager<F, R> {
    pub fn new(factory: F) -> Self {
        Self {
            factory,
            shaders: HashMap::with_capacity(10),
        }
    }

    pub fn load_shader<P: AsRef<Path>>(
        &mut self,
        v_shader_path: &P, p_shader_path: &P, g_shader_path: Option<&P>,
        name: String)
        -> Result<handle::Program<R>>
    {
        let program = self.load_shader_from_file(
            v_shader_path, p_shader_path, g_shader_path)?;
        self.shaders.insert(name, program.clone());
        Ok(program)
    }

    pub fn load_shader_from_file<P: AsRef<Path>>(
        &mut self,
        v_shader_path: &P, p_shader_path: &P, g_shader_path: Option<&P>)
        -> Result<handle::Program<R>>
    {
        let vs_code = read_code(v_shader_path)?;
        let ps_code = read_code(p_shader_path)?;

        let shader_set = match g_shader_path {
            None => {
                self.factory.create_shader_set(&vs_code, &ps_code)?
            },

            Some(g_shader_path) => {
                let gs_code = read_code(g_shader_path)?;
                self.factory.create_shader_set_geometry(
                    &vs_code, &gs_code, &ps_code)?
            }
        };

        Ok(self.factory.create_program(&shader_set)?)
    }
}

fn read_code<P: AsRef<Path>>(path: &P) -> Result<Vec<u8>> {
    use std::fs;
    use std::io;
    use std::io::Read;

    let size = fs::metadata(path)?.len() as usize;
    let mut reader = io::BufReader::new(fs::File::open(path)?);
    let mut data = Vec::with_capacity(size);
    reader.read_to_end(&mut data)?;
    Ok(data)
}
