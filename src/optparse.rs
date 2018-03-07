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
    pub help:        bool,
    pub files:       Vec<String>,
    pub version:     bool,
    pub verbose:     bool,
    pub lighting:    bool,
    pub inverted:    bool,
    pub target_size: u64,
    pub transparent: bool,
    pub color_doors: bool,
}


impl Options {
    // read args from std::env::args(), parse them
    pub fn new() -> Result<Options, String> {
        let mut arg_iter = args();
        arg_iter.next(); // push the binary path off

        if arg_iter.len() == 0 {
            return Err(format!("No args supplied"));
        }

        // all toggle-able fields for the Options struct
        let mut help          = false;
        let mut verbose       = false;
        let mut version       = false;
        let mut t_size : u64  = 1024; // TODO: this thingy
        let mut transparent   = false;
        let mut lighting      = false;
        let mut color_doors   = false;
        let mut inverted      = false;
        let mut files_buf: Vec<String> = Vec::new();


        // loop through all arguments and toggle options when detected
        let length     : usize = arg_iter.len();
        let mut index  : usize = 0;
        while index < length {
            // unpack the first argument into a local value
            let v = match arg_iter.next() {
                Some(arg) => arg,
                None      => { return Err(format!("???")); },
            };

            match v.as_str() {
                "-h"            => { help = true; }
                "--help"        => { help = true; }
                "-v"            => { version = true; }
                "--version"     => { version = true; }
                "-V"            => { verbose = true; }
                "--verbose"     => { verbose = true; }
                "-l"            => { lighting = true; }
                "--lighting"    => { lighting = true; }
                "-i"            => { inverted = true; }
                "--invert"      => { inverted = true; }
                "-d"            => { color_doors = true; }
                "--doors"       => { color_doors = true; }
                "-t"            => { transparent = true; }
                "--transparent" => { transparent = true; }

                // the next two options mirror eachother (no real way to work around this :s)
                "-s"            => {
                    let v2 = match arg_iter.next() {
                        Some(arg) => arg,
                        None      => { return Err(format!("No size arg supplied")); },
                    };

                    t_size = match v2.as_str().parse::<u64>() {
                        Ok(i) => i,
                        _     => { return Err(format!("Err: Couldn't parse '{}' to uint", v2)); }
                    };
                    index += 1;
                }
                "--size"        => {
                    let v2 = match arg_iter.next() {
                        Some(arg) => arg,
                        None      => { return Err(format!("No size arg supplied")); },
                    };
                    
                    t_size = match v2.as_str().parse::<u64>() {
                        Ok(i) => i,
                        _     => { return Err(format!("Err: Couldn't parse '{}' to uint", v2)); }
                    };
                    index += 1;
                }
                _               => { files_buf.push(v.to_string()); }
            }

            index += 1;
        }

        Ok(Options {
            help:        help,
            files:       files_buf,
            version:     version,
            verbose:     verbose,
            lighting:    lighting,
            inverted:    inverted,
            target_size: t_size,
            transparent: transparent,
            color_doors: color_doors,
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
