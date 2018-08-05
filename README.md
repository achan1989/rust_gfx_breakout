
# rust_gfx_breakout

**TL;DR** I tried building something with gfx-rs (pre-ll) but I couldn't find any easy-to-understand examples of how to do that.  I ported a game to learn how to do it, and to help teach others -- this is the result.

## Background

While trying to re-learn OpenGL, and apply it to gfx-rs, I struggled to find the right kind of learning material for the gfx-rs side of things.  It was either too basic -- draw a triangle -- or too complicated -- here's a complete thing, with no explanation of how or why we got here.

Luckily I found some excellent resources for the OpenGL side of things.  Joey de Vries of https://learnopengl.com has written a comprehensive introduction to OpenGL and graphics programming concepts, and a step-by-step tutorial about how you can apply that to making a simple Breakout game.  The tutorial is written in C++ and uses OpenGL directly.

## What is This?

To help myself learn graphics programming and the gfx library, I took the Breakout game featured in the tutorial and ported it to Rust and gfx-rs.

I built up the code by following along with the tutorial, and you can use this project's Git tags to see what happened at each part.  I tried to leave comments explaining what is being done and why -- hopefully this is helpful to others who know little about graphics programming, or nothing about gfx-rs.

Note that this uses the pre-low-level (pre-ll) branch of gfx from https://crates.io/crates/gfx.  As far as I know, development of this is now minimal -- the main development effort is focused on making a lower level API which is very much like Vulkan.  The pre-ll branch still works fine though, is being used by many current projects, and is much closer to something like OpenGL.

## What is This Not?

 - Perfect or complete.  Explanation of some of gfx-rs' concepts are still lacking.
 - Guaranteed to be the very best use of gfx-rs.  It works, but there might be better ways to do things.
 - Guaranteed to be idiomatic Rust (though I think it's pretty close).
 - A game engine.  gfx-rs is only good for experimenting with low level graphics, or for writing your own engine.

Leave me feedback and I might improve it ;)
