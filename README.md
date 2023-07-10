<p align="center">
    <img width="395" height="190" src="https://github.com/Maksasj/berryville/blob/master/assets/textures/background_logo_big.png"/>
</p>

**Berryville** - is a farming simulator game, but roles are reversed, now you are playing as a plant. Just spend your time growing, as just a simple plant 😘. Game written in Rust using Bevy game engine.

**Entry for [GMTK Game Jam 2023](https://itch.io/jam/gmtk-2023)**

> And yet, yet again, berryville is a not a new javascript framework !

### Links:
1. Web version available at [maksasj.github.io/berryville](https://maksasj.github.io/berryville/) <br>
2. Game page on Itch.io [maksasj.itch.io/berryville](https://maksasj.itch.io/berryville) <br>
3. Source code avaiable at [github.com/Maksasj/berryville](https://github.com/Maksasj/berryville)

Cool looking widgets: 
<img src="https://img.shields.io/github/license/Maksasj/berryville" alt="license">
<img src="https://img.shields.io/github/v/release/Maksasj/berryville" alt="version">
<img src="https://img.shields.io/github/actions/workflow/status/Maksasj/berryville/rust_release.yml?label=build" alt="rust_build">
<img src="https://img.shields.io/github/actions/workflow/status/Maksasj/berryville/web_release.yml?label=web build" alt="web_build">

## Building
### Requirements
Initially project have been build with these versions
1. cargo 1.70.0
2. rustc 1.70.0
3. wasm-bindgen 0.2.86 (required only for web build)

### Windows build
There is two main building options first one is building `exe` file, and second one is a `web` version.
As for `exe` version, you simply can do default or use preexisting `build.bat` and `make.bat` script.
```bash
cargo build --release 
```
Cargo will automatically, download and build dependencies, such as bevy.

### Web build
As for the web version, you will need to have `wasm-bindgen` cli utility(see this guide [link](https://rustwasm.github.io/wasm-bindgen/reference/cli.html)). After installing `wasm-bindgen`, you can try to run 
```bash
cargo build --release --target wasm32-unknown-unknown
```
this command will compile game into a wasm file. Then you can follow this guide [link](https://bevy-cheatbook.github.io/platforms/wasm.html). For development simplicity there is also `web-build` script, that compiles game into a wasm file, and runs `wasm-bindgen` automatically.

## License
Berryville is free, open source game. All code in this repository is licensed under
- MIT License ([LICENSE.md](https://github.com/Maksasj/berryville/blob/master/LICENSE.md) or https://opensource.org/license/mit/)
