![GitHub Repo Size](https://img.shields.io/github/repo-size/AgustinMDominguez/bevy-snake-rs)

# Snake


This is a toy game to learn about game development, the Rust programming language, and the game engine Bevy.

## How to Run

### Install Rust enviroment

If you already have Rust in your system, you can skip this step.

To see if you have Rust, run:

```bash
cargo --version
```

---

in linux distributions

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

for other distributions follow

- [rust-lang.org installation steps](https://www.rust-lang.org/learn/get-started)

### Install Bevy dependencies

We don't need to download the Bevy repository, since we're using it as a library and the crate is already on crates.io
We only need to install the dev dependencies for compilation

In Debian derived systems:

```bash
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev
```

For other distros and operating systems follow

- [Bevy setup](https://bevyengine.org/learn/book/getting-started/setup/)

### Build the project

Download the source code

```bash
git clone https://github.com/AgustinMDominguez/bevy-snake-rs.git
cd bevy-snake-rs
```

Compile the project. (If you didn't build a Bevy project before in your machine, this step might take a few minutes since it has to build the game engine as well.)

```bash
cargo build
```

### Run Game

```bash
cargo run
```

## Engine

The Game Engine chosen is [**Bevy**](https://bevyengine.org/), build in Rust.

Bevy a is 2D/3D Game engine in very early development, and at time of writing it is the most downloaded high-level Game Engine written in Rust with ~578K downloads. The only crate/library that has higher downloads that pertains to Game Development is [sdl2](https://crates.io/crates/sdl2), which is a library with low-level bindings to C components of SDL.

> **SDL:**
> *Simple DirectMedia Layer is a cross-platform development library designed to provide low level access to audio, keyboard, mouse, joystick, and graphics hardware via OpenGL and Direct3D.*

# Changelog

- `0.5.0`: First build that didn't have any glaring bugs.
- `0.5.1`: bugfix:
  - The snake wasn't aligned with the grid
- `0.5.2`: bugfix:
  - The second piece of food didn't spawn
- `0.5.3`: bugfix:
  - The game crashed when the snake hit a wall
- `0.6.0`: Implement boost hiting 'space'
- `0.7.0`: Implement Score

# Features

- [X] Basic snake game
- [X] Score
- [ ] Game over screen
- [ ] Restart button
- [ ] Variable speed

## Nice to have

- [X] Speed up with hold button
- [ ] Sound effects

# Known bugs

- [X] Game crashes when snake hits a wall
- [X] Sometimes food doesn't spawn
- [ ] Sometimes the games panics on spawn
