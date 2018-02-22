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

extern crate gfx;
extern crate glfw;


error_chain! {
    // Declare that we want to convert these non-error-chain errors into
    // error-chain errors.
    foreign_links {
        GlfwInit(glfw::InitError);
        Io(::std::io::Error);
        ProgramError(gfx::shade::ProgramError);
        CreateProgramError(gfx::shade::core::CreateProgramError);
    }
}
