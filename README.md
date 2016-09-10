# BOXES CATCHING BOXES

You are a box. Fullfill your dream of catching other boxes, for some reason...

## How to run

You can clone this project, make sure that you have [sdl2](https://www.libsdl.org/download-2.0.php), [sdl2_ttf](https://www.libsdl.org/projects/SDL_ttf/), and [sdl2_mixer](https://www.libsdl.org/projects/SDL_mixer/) installed on your system.

You may also need to make sure that all of these have their libs linked in your LIBRARY_PATH used by rustc. They were all linked in my own /usr/local/lib directory which was already added to the LIBRARY_PATH variable in my shell, but your configuration may be different.

Also make sure you have a recent version of [Rust](https://www.rust-lang.org/en-US/) installed. I'm currently running 1.11.0.

Once you have all of this setup give `cargo run` in the root directory a whirl. Let me know if something doesn't work as I'm currently still learning about how to develop cross-platform Rust applications, and will want to figure out how to work through all the various kinks.

## Instructions

Use left and right to move your box, touch the falling boxes to catch them. Press Escape to quit when you have sufficiently filled said dreams.

## Purpose

This is a very small experiment just to relearn rust.

## TODO
*   Get the types right so I'm not casting all over the place.
*   Add a main menu.
