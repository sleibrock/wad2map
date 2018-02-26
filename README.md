# wad2map - Maps for WADs

`wad2map` is a [Rust](https://rust-lang.org/) program written to convert *[Doom](https://en.wikipedia.org/wiki/Doom_(1993_video_game))* maps into Scalable Vector Graphic files.


## Installation

To install, use the following set of instructions:
```bash
git clone https://github.com/sleibrock/wad2map && cd wad2map
cargo install
```

You need to use Rust's Cargo tool, so make sure you install Rust with [Rustup](https://www.rustup.rs/).

## Instructions

`wad2map` accepts a path to a WAD file and will render all maps in the WAD into a new folder the same name as the WAD. `wad2map` also accepts multiple WADs at once for batch rendering.

If you wanted to generate all of Doom 1's maps into SVG format, simply call the following instruction:
```bash
wad2map doom.wad
```

To run `wad2map` on a list of files, simply list each file sequentially.
```bash
wad2map doom.wad doom2.wad heretic.wad ...
```

### Converting SVGs to PNG

`wad2map` exports all levels in Scalable Vector Graphics format to preserve quality when scaling the image upwards. In order to convert the SVG to something like Portable Network Graphics (PNG), you can use the standard Linux tool `convert` to convert a map to PNG format.

```bash
$ convert doom.wad.maps/E1M1.svg ./E1M1.png
```

Optionally you can use GIMP or Inkscape to also do similar things.


### Examples

You can see examples in the [examples directory](https://github.com/sleibrock/wad2map/tree/master/examples) which contains different wads that I've tested (all IWADs, some PWADs).

Doom's E1M1 "Hangar"
![Doom E1M1](https://raw.githubusercontent.com/sleibrock/wad2map/master/examples/doom/E1M1.png)

Doom II's MAP01 "Entryway"
![Doom 2 MAP01](https://raw.githubusercontent.com/sleibrock/wad2map/master/examples/doom2/MAP01.png)

Heretic's E1M1 "The Docks"
![Heretic E1M1](https://raw.githubusercontent.com/sleibrock/wad2map/master/examples/heretic/E1M1.png)


