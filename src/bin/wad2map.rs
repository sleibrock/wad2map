// wad2map.rs

extern crate wad2map;

use std::env;
use std::process::exit;

use wad2map::parse_wad::parse_wad;
use wad2map::mapmaker::make_maps_from_wad;

fn main() {
    let mut arg_iter = env::args();
    arg_iter.next();

    // early exit if no args supplied
    if arg_iter.len() == 0 {
        println!("Args list empty!");
        exit(-1);
    }

    // loop through all arguments and parse a wad from each one
    for arg in arg_iter {
        let fname = format!("{}", arg).to_owned();
        
        let wad = match parse_wad(&fname) {
            Ok(new_wad) => new_wad,
            _ => { panic!("HELP"); }
        };

        make_maps_from_wad(&fname, &wad);
    }

    exit(0);
}

// end
