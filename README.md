# SCOP

## Requirement
- rustc: 1.75.0

### for MacOS
```
$ brew install sdl2
$ sdl2-config --libs
-L/opt/homebrew/lib -lSDL2
# better to write inside ~/.zshrc
$ export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"
```
> for more detail: https://github.com/Rust-SDL2/rust-sdl2

- https://crates.io/crates/sdl2