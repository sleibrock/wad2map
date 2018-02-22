// mapmaker.rs
// TODO: make the drawing algorithm a lot better


use std::fs::create_dir;

use svg::*;
use doom::level::*;
use doom::wad::*;


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
    format!("{}/{}.svg", dir, lname)
}


pub fn normalize(a: i32, b: i32, c: i32) -> i32 {
    (a+b+c)
}

pub fn flip(v: i32, m: i32) -> i32 {
    let delta = (m-v).abs();
    if v < m {
        return v + (2*delta);
    } else {
        return v - (2*delta);
    }
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

    // padding from the edge of the image
    let padding : i32 = 10;


    // create a canvas and start adding objects to it
    let view_box_x = normalize(max_x as i32, shift_x, 10) as u64;
    let view_box_y = normalize(max_y as i32, shift_y, 10) as u64;

    let mut buf = SVG::new(1024, 768, view_box_x, view_box_y);

    let mx = view_box_x / 2;
    let my = view_box_y / 2;

    for linedef in &lev.linedefs {
        let a = &lev.vertices[linedef.start as usize];
        let b = &lev.vertices[linedef.end as usize];

        let ax = normalize(a.x as i32, shift_x, 0) as i32;
        let ay = flip(normalize(a.y as i32, shift_y, 0), my as i32);
        let bx = normalize(b.x as i32, shift_x, 0) as i32;
        let by = flip(normalize(b.y as i32, shift_y, 0), my as i32);
        let l = SVGLine::new(
            ax as u64, ay as u64, bx as u64, by as u64,
            5,
            Color::Black
        );

        buf.add_object(Box::new(l));
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
        let output_path = make_path_str(&wad_dir_name, &lev.name);

        match svg_thing.to_file(&output_path) {
            Ok(_)  => { println!("File saved!"); },
            Err(e) => { panic!(format!("Error: {}", e)); }
        }
    }

    0
} 
