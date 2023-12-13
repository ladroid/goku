# Roguelike game

Randomly generated roguelike game

## Instructions
* Move - Arrows (up, down, left, right)
* Attack - Space

## Key features:
* Randomly generated dungeon.
* Point Light.
* Randomly spawning enemies.
* Enemies follow player.
* Infinite levels.
* Enemies can attack player.
* Simple health bar.
* Playing audio. 
* Minimap.

~~**Note:** However approach is not good since I am using [rodio library.](https://github.com/RustAudio/rodio) Well the reason is because it is not possible(it will take some time for me) on Sdl2 make a thread-safety for playing music on a background and at the same time rendering UI part (check [this](https://github.com/Rust-SDL2/rust-sdl2/issues/1063)). I am trying to solve it but now let it be this approach. However AudioPlayer is working but only when we have a one thread.~~

## Source code
Source code can be shown [here](/examples/roguelike/game_example.rs)

## Assets
Character was taken from [here](https://totuslotus.itch.io/characterpack)

## Where to play
The game is located on itch.io (soon will be the link).