// mapmaker.rs
// TODO: make the drawing algorithm a lot better


use std::fs::create_dir;
use svg::*;
use doom::level::*;
use doom::wad::*;


// map a string (most likely a filepath for a wad) to a folder path string
pub fn dir_name(dname: &str) -> String {
    format!("{}.maps", dname)
}


// create a directory and return whether it was made or not
pub fn make_directory(dname: &str) -> bool {
    match create_dir(format!("{}", dname)) {
        Ok(_) => true,
        _     => false,
    }
}


// map a file name (level name) to an output file location string
pub fn make_path_str(dir: &str, lname: &str) -> String {
    format!("{}/{}.svg", dir, lname)
}


// flip a value around a middle point
pub fn flip(v: i32, m: i32) -> i32 {
    let delta = (m-v).abs();
    match v < m {
        true  => v + (2*delta),
        false => v - (2*delta),
    }
}


// convert a &Level into an SVG Buffer
// calculates a lot of numbers and converts LineDefs into SVGLine objects
pub fn level_to_svg(lev: &Level) -> SVG {
    // iterate through all vertices to find min/max bounds
    let mut min_x : i16 = 0;
    let mut min_y : i16 = 0;
    let mut max_x : i16 = 0;
    let mut max_y : i16 = 0;
    for vert in &lev.vertices {
        if min_x == 0 && max_x == 0 && min_y == 0 && max_y == 0 {
            // set the min/max vars to the first vertex
            min_x = vert.x; max_x = vert.x;
            min_y = vert.y; max_y = vert.y;
        } else {
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
    }

    // determine the shift offset to keep everything in 0..65535 range
    let shift_x : i32 = 0 - min_x as i32;
    let shift_y : i32 = 0 - min_y as i32;

    // padding from the edge of the image
    let padding : u64 = 50;

    // viewbox numbers (total domain of all vertices [0..65535])
    let vx = (max_x as i32) + shift_x + (2*padding as i32);
    let vy = (max_y as i32) + shift_y + (2*padding as i32);

    // calculate the image canvas size by using the aspect ratio of the viewbox numbers
    let base_canvas_size : f64 = 1024.0;
    let cx : u64;
    let cy : u64;
    if vx > vy {
        let ratio = base_canvas_size / vx as f64;
        cx = (vx as f64 * ratio) as u64;
        cy = (vy as f64 * ratio) as u64;
    } else {
        let ratio = base_canvas_size / vy as f64;
        cx = (vx as f64 * ratio) as u64;
        cy = (vy as f64 * ratio) as u64;
    }

    let mut buf = SVG::new(cx, cy, vx as u64, vy as u64);
    buf.add_object(Box::new(SVGRect::new(0, 0, vx as u64, vy as u64, Color::White)));

    //let mx = ((max_x as i32) + shift_x) / 2;
    let my = ((max_y as i32) + shift_y) / 2;

    for linedef in &lev.linedefs {
        let a = &lev.vertices[linedef.start as usize];
        let b = &lev.vertices[linedef.end   as usize];

        let ax = (a.x as i32) + shift_x; let ay = (a.y as i32) + shift_y;
        let bx = (b.x as i32) + shift_x; let by = (b.y as i32) + shift_y;

        // y values need to be flipped
        let l = SVGLine::new(
            padding + (ax as u64),
            padding + (flip(ay, my) as u64),
            padding + (bx as u64),
            padding + (flip(by, my) as u64),

            match linedef.is_one_sided() {
                true => 7,
                _    => 5,
            },

            // if a linedef is one sided, it means it cannot be passed through
            match linedef.is_one_sided() {
                true => Color::Black,
                _    => Color::Grey,
            }
        );

        buf.add_object(Box::new(l));
    }
    return buf;
}


// Take a &Wad and start converting all it's levels to SVG buffers
// Using said buffers, write each one to a corresponding file
pub fn make_maps_from_wad(fname: &str, wad: &Wad) -> u8 {
    let wad_dir_name = dir_name(fname);
    let dir_made = make_directory(&wad_dir_name);
    if dir_made {
        println!("Directory made!");
    }

    for lev in &wad.levels {
        let mut svg_thing = level_to_svg(&lev);
        let output_path = make_path_str(&wad_dir_name, &lev.name);

        match svg_thing.to_file(&output_path) {
            Ok(_)  => {},
            Err(e) => { panic!(format!("Error: {}", e)); }
        }
    }
    return 0;
} 


// end
