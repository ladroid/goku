# goku

![GitHub Workflow Status](https://img.shields.io/github/commit-activity/t/ladroid/goku)
[![GitHub Repo stars](https://img.shields.io/github/stars/ladroid/goku)](https://github.com/ladroid/goku)
[![Documentation](https://docs.rs/imgui-wgpu/badge.svg)](https://lados-organization.gitbook.io/goku/)
![Repo Size](https://img.shields.io/github/repo-size/ladroid/goku)
![License](https://img.shields.io/github/license/ladroid/goku)

[ [English](README.md) | [Deutsch](docs/README.de.md) | [Español](docs/README.es.md) | [Français](docs/README.fr.md) | [日本語](docs/README.ja.md) ]


goku is a 2D game development application for Rust (in the future integration with 3D games). Written purely in Rust. 

It’s available for **macOS**, **Windows** and **Linux**.

Based on SDL2 (currently).

goku is focused, lightweight and has few dependencies (mostly SDL2). It provides:

* a window and a main loop

* 2D graphics and text

* sounds and music

* keyboard, mouse, and gamepad input

* GUI interface for development

<ins>third party libraries which used goku currently:</ins>

* SDL2

* nalgebra

* imgui

* serde

* rfd

* glow

* webbrowser

## How to use

**IMPORTANT!!!**

The documentation is located here -> [Gitbook](https://lados-organization.gitbook.io/goku/)

## Features

* Graphics:
    * a window and a main loop

    * 2D graphics and text 
        - Button 
        - Slider 
        - Checkbox
        - TextBox
        - Fonts (only ttf format)
        - Particle system (sparks)
        - Parallax background

    * GUI interface for development
        * built-in Text Editor
        * Debug Console

    * Multiple image file formats: JPG and PNG

* Audio
    - Play
    - Loop
    - Pause
    - Resume
    - Multiple audio file formats: OGG, MP3, FLAC, MOD

* Input handler:
    * keyboard, mouse, and gamepad input

* Math types:
    * Vector2 Vector3, Vector4
    * Matrix33, Matrix34, Matrix43, Matrix44

* Physics:
    * Collisions
    * Rigid body (currently we don't have a Kinematic rigid body)

* Scene:
    * Flexible JSON file format: Could describe either a whole scene or individual meshes.

* Animation

* AI system:
    * Behaviour Tree

* Timer

* Dialogue System

* Supports several languages:
    - German
    - Spanish
    - Japanese
    - French

* Support Platform:
    - Windows / Mac / Linux
    - Web (WASM not integrated fully) (addition references [Emscripte](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/) / [SDL-WASM](https://gitlab.com/ThibaultLemaire/rust-sdl-canvas-wasm) / [Web](https://github.com/koute/cargo-web) )
    - Android in the future

## How to run

command to run: `cargo run`

**Important!!!** Currently GUI is still under the development, I'm trying to combine imgui and sdl2 together but need sometime to make it full compatible. Thus, if someone wants to use it is possible to add component `Scene` and write there a script. If you know how to combine them will be really great!

## How to build for Web

1. Press Tools
2. Press Build
3. run this command in the directory where it was build `cargo web start wasm32-unknown-emscripten` or `cargo web build --target wasm32-unknown-emscripten`

## TODO (is prioritised)

* make a viewport instead of current solution with canvas (probably need a separate window inside of app with combination of sdl2 and imgui)

* improve physics

* add/improve UI system (make possible to add image for buttons and rounded buttons)

* improve lights and shadows

* draw simple shapes (cirlce, rectangle, triangle, etc.)

* tabs

* profiler

* add/improve particle system

* improve engine's GUI as well as text editor (probably instead of built-in text editor make an integration with VSCode or other ide)

* add blueprints (probably imgui node graph https://github.com/benmkw/imnodes-rs)

* building games for mobiles iOS, Android

* building games for consoles (PS4-5), Xbox, Nintendo Switch

* physics material

* integration with C++ (probably something like a bindgen)

## Examples

### 1. Tetris

Example of building tetris game can be found [here](examples/tetris_game_example.txt)

### 2. Roguelike prototype (TODO)

Example of building roguelike prototype can be found here -> https://github.com/ladroid

### 3. Visual effects

1. Sparks -> simply use a function
2. Fire -> simply use a function
3. Rain -> simply use a function

### 4. Side scrolling game

Example of building sie scrolling prototype can be found [here](examples/simple_parallax_example.txt)

### 5. Platformer

Example of building platformer prototype can be found here -> https://github.com/ladroid

### 6. Set simple states for enemy(chasing/following)

Example of building platformer prototype can be found here -> https://github.com/ladroid

## Japanese ver.

Japenese version can be found [here](https://lados-organization.gitbook.io/goku/v/goku-game-engine_jp/)

## French ver.

French version can be found [here](https://lados-organization.gitbook.io/goku/v/goku-game-engine_fr/)

## German ver.

German version can be found [here](https://lados-organization.gitbook.io/goku/v/goku-game-engine_de/)

## Spanish ver.

Spanish version can be found [here](https://lados-organization.gitbook.io/goku/v/goku-game-engine_es/)

## How to Contribute

### Submitting Issues
Use the Issue Tracker to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing
Anyone manually testing the code and reporting bugs or suggestions for enhancements in the Issue Tracker are very welcome!

### Submitting Pull Requests
Patches/fixes are accepted in form of pull requests (PRs). Make sure the issue the pull request addresses is open in the Issue Tracker.

Submitted pull request is deemed to have agreed to publish under Apache 2.0 License.

## Community

[Discord](https://discord.gg/9TAMqdRyED)

[GitHub Discussion](https://docs.github.com/en/discussions/quickstart)

## License
goku is licensed under Apache license version 2.0. See [LICENSE](https://pages.github.com/) file.
