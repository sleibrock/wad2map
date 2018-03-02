// mapmaker.rs
// TODO: make the drawing algorithm a lot better


use std::fs::create_dir;
use svg::*;
use optparse::Options;
use doom::linedef::*;
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


// flip a value in a certain axis
// if the axis is set to zero, just return the initial value
pub fn flatten(v: u64, m: u64) -> u64 {
    if m == 0 {
        return v;
    }
    let d = m - v;
    return d;
}


// Given a line, determine it's color
// Whether it's a key door, wall, or two-sided line
pub fn line_color(line: &LineDef, color_doors: bool) -> Color {
    // if coloring doors is false, just stick to default
    if !color_doors {
        return match line.is_one_sided() {
            true => Color::Black,
            _    => Color::Grey,
        };
    }

    // Check the value against all key door values
    match line.special_type() {
        28 => Color::Red,    // red keycard
        33 => Color::Red,    // red keycard stay open
        26 => Color::Blue,   // blue keycard
        32 => Color::Blue,   // blue keycard stay open
        27 => Color::Yellow, // yellow keycard
        34 => Color::Yellow, // yellow keycard stay open

        // else, check if it's a wall or not
        _  => match line.is_one_sided() {
            true => Color::Black,
            _    => Color::Grey,
        }
    }
}


// convert a &Level into an SVG Buffer
// calculates a lot of numbers and converts LineDefs into SVGLine objects
pub fn level_to_svg(lev: &Level, opts: &Options) -> SVG {
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
    let shift_x = 0 - min_x as i32;
    let shift_y = 0 - min_y as i32;

    // padding from the edge of the image
    let padding : u64 = 50;

    // numbers that define the max X and Y ranges
    let mx = (max_x as i32) + shift_x;
    let my = (max_y as i32) + shift_y;

    // viewbox numbers that include the padding for the image
    let vx = mx + (2*padding as i32);
    let vy = my + (2*padding as i32);

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

    // check if we want a transparent background
    // if not, add a white background matching the dimensions
    if !opts.transparent {
        buf.add_object(Box::new(SVGRect::new(0, 0, vx as u64, vy as u64, Color::White)));
    }

    for linedef in &lev.linedefs {
        let a = &lev.vertices[linedef.start as usize];
        let b = &lev.vertices[linedef.end   as usize];

        let ax = ((a.x as i32) + shift_x) as u64;
        let ay = ((a.y as i32) + shift_y) as u64;
        let bx = ((b.x as i32) + shift_x) as u64;
        let by = ((b.y as i32) + shift_y) as u64;

        buf.add_object(Box::new(SVGLine::new(
            padding + flatten(ax, 0),
            padding + flatten(ay, my as u64),
            padding + flatten(bx, 0),
            padding + flatten(by, my as u64),

            // if a linedef is one-sided use differentiating colors and widths
            match linedef.is_one_sided() {
                true => 7,
                _    => 5,
            },
            line_color(linedef, opts.color_doors),
        )));
    }
    return buf;
}


// Take a &Wad and start converting all it's levels to SVG buffers
// Using said buffers, write each one to a corresponding file
pub fn make_maps_from_wad(fname: &str, wad: &Wad, opts: &Options) -> u8 {
    let wad_dir_name = dir_name(fname);
    let dir_made = make_directory(&wad_dir_name);
    if dir_made  && opts.verbose {
        println!("Directory made!");
    }

    for lev in &wad.levels {
        let mut svg_thing = level_to_svg(&lev, opts);
        let output_path = make_path_str(&wad_dir_name, &lev.name);

        match svg_thing.to_file(&output_path) {
            Ok(_)  => {},
            Err(e) => { panic!(format!("Error: {}", e)); }
        }
    }

    if opts.verbose {
        println!("Finished rendering maps for {}", fname);
    }
    return 0;
} 


// end
