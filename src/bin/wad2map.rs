// wad2map.rs

extern crate wad2map;

use std::process::exit;

use wad2map::optparse::Options;
use wad2map::parse_wad::parse_wad;
use wad2map::mapmaker::make_maps_from_wad;

fn main() {
    // generate an Options struct reading args from CLI
    let opts = match Options::new() {
        Ok(opt) => opt,
        Err(ec) => {
            println!("{}", ec);
            exit(-2);
        }
    };

    // do a bunch of checks and then exit with status codes
    if opts.help {
        opts.print_help();
        exit(0);
    }

    if opts.version {
        opts.print_version();
        exit(0);
    }

    if opts.files.len() == 0 {
        println!("No files supplied");
        exit(-1);
    }

    // loop through all arguments and parse a wad from each one
    let mut passes : usize = 0;
    let mut fails  : usize = 0;
    for file in &opts.files {
        let fname = format!("{}", file).to_owned();

        match parse_wad(&fname, &opts) {
            Ok(new_wad) => match make_maps_from_wad(&fname, &new_wad, &opts) {
                Ok(_) => { passes += 1; },
                Err(e) => {
                    if opts.verbose {
                        println!("make_maps_from_svg: {}", e);
                    }
                    fails += 1;
                }
            },
            Err(e) => {
                if opts.verbose {
                    println!("parse_wad: {}", e);
                }
                fails += 1;
            }
        }
    }

    if opts.verbose {
        println!("{} file(s) rendered, {} file(s) failed", passes, fails);
    }

    exit(0);
}

// end
