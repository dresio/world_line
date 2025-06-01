![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)

# World Line

This is a game for submission in the Bevy 6 Game Jam with the theme of Chain Reaction.

## Theme

The theme of this game jam is Chain Reaction, and I am hoping to hit this in two different ways. One is extremely straight forward with lots of explosions and such literally creating chain reactions, and a deeper meaning by making the interactions you have with the world impact the story

## Running

This utilizes the [Bevy CLI](https://github.com/TheBevyFlock/bevy_cli) for runtime. Use `bevy run --release web --open` to run it in your local browser. This should also be accessible on Itch.io at TO COME SOON (currently just deployed to github pages).

## Extra Crates

This project is planning on utilizing a handful of external crates to make development more streamlined for the game jam.

- [skein](https://github.com/rust-adventure/skein): Helps with loading 3D assets and level design
- [avian](https://github.com/Jondolf/avian): Provides physics engine
- [bevy-tnua](https://github.com/idanarye/bevy-tnua): Character controller compatible with the avian crate
- [bevy_enhanced_input](https://github.com/projectharmonia/bevy_enhanced_input): Robust input handling

## License

Following in the footsteps of Bevy and other rust crates, world_line is dual licensed under <a href="LICENSE-APACHE.txt">Apache License, Version
2.0</a> or <a href="LICENSE-MIT.txt">MIT license</a> which you can choose either.  
Please note some crates this depends on may have other license, which can typically be found in their README files underneath the License header.
