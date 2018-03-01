// optparse.rs

use std::env::args;

/// This is where command line options are parsed
/// Turn the CLI options into a Struct for pass-through to various functions

pub struct Options {
    pub help:               bool,
    pub verbose:            bool,
    pub target_size:         u64,
    pub transparent:        bool,
    pub lighting:           bool,
    pub color_doors:        bool,
    pub files:       Vec<String>,
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
        let target_size     = 1024; // TODO: this thingy
        let mut transparent = false;
        let mut lighting    = false;
        let mut color_doors = false;
        let mut files_buf : Vec<String> = Vec::new();

        // loop through all args and match for values
        for arg in arg_iter {
            match arg.as_str() {
                "-h"            => { help = true; },
                "--help"        => { help = true; },
                "-v"            => { verbose = true; },
                "--verbose"     => { verbose = true; },
                "-t"            => { transparent = true; },
                "--transparent" => { transparent = true; },
                "-l"            => { lighting = true; },
                "--lighting"    => { lighting = true; },
                "-d"            => { color_doors = true; },
                "--doors"       => { color_doors = true; },
                _               => { files_buf.push(arg.to_string()); },
            }
        }

        Ok(Options{
            help:               help,
            verbose:         verbose,
            target_size: target_size,
            transparent: transparent,
            lighting:       lighting,
            color_doors: color_doors,
            files:         files_buf,
        })
    }


    // print a help command when ran with -h
    pub fn print_help(&self) {
    }
}

// end
