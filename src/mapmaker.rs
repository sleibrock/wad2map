// logic behind converting a Wad struct to a series of SVG file outputs

use std::fs::create_dir;

use svg::*;
use doom::level::*;
use doom::wad::*;
use doom::lump::*;


pub fn dir_name(dname: &str) -> String {
    format!("{}.maps", dname)
}


pub fn make_directory(dname: &str) -> bool {
    match create_dir(format!("{}", dname)) {
        Ok(_) => true,
        _     => false,
    }
}


pub fn make_path_str(dir: &str, lname: &str) -> String {
    format!("{}/{}", dir, lname)
}


pub fn normalize(a: i16, b: i16, c: i16) -> i16 {
    a+b+c
}


pub fn level_to_svg(lev: &Level) -> SVG {

    // iterate through all vertices to find min/max bounds
    let mut min_x : i16 = 0;
    let mut min_y : i16 = 0;
    let mut max_x : i16 = 0;
    let mut max_y : i16 = 0;

    for vert in &lev.vertices {
        if vert.x > max_x {
            max_x = vert.x;
        } else if vert.x < min_x {
            min_x = vert.x;
        }

        if vert.y > max_y {
            max_y = vert.y;
        } else if vert.y < min_y {
            min_y = vert.y;
        }
    }

    let shift_x : i32 = 0 - min_x as i32;
    let shift_y : i32 = 0 - min_y as i32;


    // create a canvas and start adding objects to it
    let mut buf = SVG::new(1024, 1024);

    for linedef in &lev.linedefs {
        //println!("a: {}", linedef.start);
        //println!("b: {}", linedef.end);
        //let a = &lev.vertices[linedef.start];

    }

    buf
}


pub fn make_maps_from_wad(fname: &str, wad: &Wad) -> u8 {
    let wad_dir_name = dir_name(fname);
    let dir_made = make_directory(&wad_dir_name);
    if dir_made {
        println!("Directory made!");
    }
    

    for lev in &wad.levels {
        lev.print();
        let mut svg_thing = level_to_svg(&lev);
        svg_thing.to_file(&make_path_str(&wad_dir_name, &lev.name));
    }


    0
} 
