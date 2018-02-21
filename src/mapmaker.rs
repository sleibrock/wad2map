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


pub fn normalize(a: i32, b: i32, c: i32) -> u64 {
    (a+b+c) as u64
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
    let view_box_x = normalize(max_x as i32, shift_x, 10);
    let view_box_y = normalize(max_y as i32, shift_y, 10);

    let aspect_ratio_bool = view_box_x > view_box_y;
    let canvas_x : u64 = match aspect_ratio_bool {
        true => 1024,
        _    => (1024.0 * (view_box_y as f64 / view_box_x as f64)) as u64,
    }; 

    let canvas_y : u64 = match aspect_ratio_bool {
        true => (1024.0 * (view_box_x as f64 / view_box_y as f64)) as u64,
        _    => 1024,
    };

    let mut buf = SVG::new(canvas_x, canvas_y, view_box_x, view_box_y);

    for linedef in &lev.linedefs {
        let a = &lev.vertices[linedef.start as usize];
        let b = &lev.vertices[linedef.end as usize];

        let ax = normalize(a.x as i32, shift_x, 0);
        let ay = normalize(a.y as i32, shift_y, 0);
        let bx = normalize(b.x as i32, shift_x, 0);
        let by = normalize(b.y as i32, shift_y, 0);
        let l = SVGLine::new(
            ax, ay, bx, by,
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
