// optparse.rs

use std::env::args;

/// This is where command line options are parsed
/// Turn the CLI options into a Struct for pass-through to various functions

const HELP_STR: &'static str = "Usage: wad2map [OPTION] ... [FILE] ...
Convert all levels from a list of WADs into SVG files
exported to matching directories of the original WAD filepath


  -h, --help         Show this help and exit
  -v, --version      Show program version and exit
  -V, --verbose      Toggle program verbosity
  -t, --transparent  Render images with no backgrounds
  -l, --lighting     Render images using sector lighting
  -i, --invert       Invert the colors (black bg, white fg)
  -s, --size [NUM]   Change the base canvas size
  -d, --doors        Color all keycard/skullkey doors

Examples:
  wad2map doom.wad        Exports all levels into './doom.wad.maps'
  wad2map -t heretic.wad  Exports all Heretic levels as transparent

More help can be found at <https://github.com/sleibrock/wad2map>
";

pub struct Options {
    pub help: bool,
    pub version: bool,
    pub verbose: bool,
    pub target_size: u64,
    pub transparent: bool,
    pub lighting: bool,
    pub color_doors: bool,
    pub files: Vec<String>,
}

impl Options {
    // read args from std::env::args(), parse them
    pub fn new() -> Result<Options, String> {
        let mut arg_iter = args();
        arg_iter.next(); // push the binary path off

        if arg_iter.len() == 0 {
            return Err(String::from("No args supplied"));
        }

        let mut help        = false;
        let mut verbose     = false;
        let mut version     = false;
        let target_size     = 1024; // TODO: this thingy
        let mut transparent = false;
        let mut lighting    = false;
        let mut color_doors = false;
        let mut files_buf: Vec<String> = Vec::new();

        // loop through all args and match for values
        // TODO: write differently to parse "--flag <value>" args
        for arg in arg_iter {
            match arg.as_str() {
                "-h"            => { help = true; },
                "--help"        => { help = true; },
                "-v"            => { version = true; },
                "--version"     => { version = true; },
                "-V"            => { verbose = true; },
                "--verbose"     => { verbose = true; },
                "-t"            => { transparent = true; },
                "--transparent" => { transparent = true; },
                "-l"            => { lighting = true; },
                "--lighting"    => { lighting = true; },
                "-d"            => { color_doors = true; },
                "--doors"       => {color_doors = true; },
                _               => { files_buf.push(arg.to_string()); },
            }
        }

        Ok(Options {
            help: help,
            version: version,
            verbose: verbose,
            target_size: target_size,
            transparent: transparent,
            lighting: lighting,
            color_doors: color_doors,
            files: files_buf,
        })
    }

    // Print out the cargo version and pkg name
    pub fn print_version(&self) {
        println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }

    // print a help command when ran with -h
    pub fn print_help(&self) {
        println!("{}", HELP_STR);
    }
}

// end
