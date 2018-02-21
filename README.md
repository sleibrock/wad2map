# wad2map - Maps for WADs

`wad2map` is a Rust program written to create maps from Doom WAD files.


## Installation

To install, use the following set of instructions:
```bash
git clone https://github.com/sleibrock/wad2map && cd wad2map
cargo install
```

You need to use Rust's Cargo tool, so make sure you install Rust with Rustup.

## Instructions

`wad2map` accepts a path to a WAD file and will render all maps in the WAD into a new folder the same name as the WAD. `wad2map` also accepts multiple WADs at once for batch rendering. 

If you wanted to generate all of Doom 1's maps into SVG format, simply call the following instruction:
```bash
$ wad2map doom.wad
$ ls doom.wad.maps
E1M1.svg  E1M9.svg  E2M8.svg  E3M7.svg  E4M6.svg
E1M2.svg  E2M1.svg  E2M9.svg  E3M8.svg  E4M7.svg
E1M3.svg  E2M2.svg  E3M1.svg  E3M9.svg  E4M8.svg
E1M4.svg  E2M3.svg  E3M2.svg  E4M1.svg  E4M9.svg
E1M5.svg  E2M4.svg  E3M3.svg  E4M2.svg
E1M6.svg  E2M5.svg  E3M4.svg  E4M3.svg
E1M7.svg  E2M6.svg  E3M5.svg  E4M4.svg
E1M8.svg  E2M7.svg  E3M6.svg  E4M5.svg
```
