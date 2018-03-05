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

// We use the error-chain crate to take care of a lot of the error handling
// boilerplate.  Whenever we see Result<_>, it's the special version from
// error-chain -- see errors.rs

use std::collections::HashMap;

#[macro_use]
extern crate error_chain;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glfw;
extern crate glfw;

extern crate rust_gfx_breakout as breakout;
use breakout::errors::*;


type EventQueue = ::std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

// gfx is a bit awkward with its types. Make some shorter aliases.
// ColourFormat and DepthFormat are specific to the kind of framebuffer we're
// using, but the others are generic and would let us use a different backend
// without changing them.
type ColourFormat = gfx::format::Rgba8;
type DepthFormat = gfx::format::DepthStencil;
type RenderTargetView<R: gfx::Resources> =
    gfx::handle::RenderTargetView<R, ColourFormat>;
type DepthStencilView<R: gfx::Resources> =
    gfx::handle::DepthStencilView<R, DepthFormat>;

const NUM_KEYS: usize = 150;  // Roughly this many keys on the keyboard.
type KeyMap = HashMap<glfw::Key, bool>;

// Let's bundle the graphics-related stuff together.
// This is also generic, and should work with any backend.
struct Gfx<C: gfx::CommandBuffer<R>, D: gfx::Device, F: gfx::traits::FactoryExt<R> + Clone, R: gfx::Resources> {
    colour_view: RenderTargetView<R>,
    depth_view: DepthStencilView<R>,
    device: D,
    encoder: gfx::Encoder<R, C>,
    factory: F,
}

fn run() -> Result<()> {
    let (events, mut glfw, mut window, mut gfx) = setup_gl_window_and_gfx()?;
    let (fb_width, fb_height) = window.get_framebuffer_size();
    let mut keys: KeyMap = KeyMap::with_capacity(NUM_KEYS);

    // TODO: set viewport.

    // Initialize game
    let mut breakout = breakout::Game::new(
        fb_width, fb_height, gfx.factory.clone(), gfx.colour_view.clone())?;

    let mut delta_time = 0.0;
    let mut last_frame = 0.0;

    while !window.should_close() {
        // Calculate delta time.
        // If we wanted to do this without relying on GLFW we could use
        // std::time::Instant.
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;
        process_events(&mut glfw, &events, &mut keys, &mut window);

        breakout.process_input(delta_time);
        breakout.update(delta_time);

        gfx.encoder.clear(&gfx.colour_view, [0.0, 0.0, 0.0, 1.0]);
        // Depth buffer is not actually used, but if it were...
        gfx.encoder.clear_depth(&gfx.depth_view, 1.0);
        breakout.render(&mut gfx.encoder);
        gfx.encoder.flush(&mut gfx.device);

        {
            use glfw::Context;
            use gfx::Device;

            window.swap_buffers();
            gfx.device.cleanup();
        }
    }

    Ok(())
}

fn process_events(glfw: &mut glfw::Glfw, events: &EventQueue,
                  keys: &mut KeyMap, window: &mut glfw::Window)
{
    use self::glfw::{Action, Key, WindowEvent};

    glfw.poll_events();
    for (_time, event) in glfw::flush_messages(&events) {
        match event {
            WindowEvent::Key(Key::Escape, _code, Action::Press, _mods) => {
                window.set_should_close(true);
            },
            WindowEvent::Key(k, _scancode, action, _mods) => {
                let pressed = match action {
                    Action::Press | Action::Repeat => true,
                    Action::Release => false
                };
                keys.insert(k, pressed);
            },

            evt => panic!("unexpected event typ {:?}", evt)
        }
    }
}

// This creates our window, event, and graphics objects, using a specific
// window/event system (GLFW) and backend (OpenGL).
fn setup_gl_window_and_gfx()
    -> Result<(
        EventQueue,
        glfw::Glfw,
        glfw::Window,
        Gfx<gfx_device_gl::CommandBuffer,
            gfx_device_gl::Device,
            gfx_device_gl::Factory,
            gfx_device_gl::Resources>)>
{
    use self::glfw::{WindowHint, OpenGlProfileHint, WindowMode};
    const SCREEN_WIDTH: u32 = 800;
    const SCREEN_HEIGHT: u32 = 600;

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    // glfw.window_hint(WindowHint::OpenGlDebugContext(true));
    // glfw.window_hint(WindowHint::ContextNoError(false));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::DoubleBuffer(true));
    glfw.window_hint(WindowHint::Resizable(false));

    let (mut window, events) =
        glfw.create_window(
            SCREEN_WIDTH, SCREEN_HEIGHT, "Breakout", WindowMode::Windowed)
        .ok_or("Failed to create GLFW window")?;

    window.set_key_polling(true);

    let (device, mut factory, colour_view, depth_view) =
        gfx_window_glfw::init(&mut window);
    let encoder = factory.create_command_buffer().into();
    let gfx = Gfx {
        colour_view,
        depth_view,
        device,
        encoder,
        factory,
    };

    Ok((events, glfw, window, gfx))
}

quick_main!(run);
