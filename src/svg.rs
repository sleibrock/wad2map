// svg.rs

// The SVG struct holds shapes and can render to file
//
// Example file:
// <?xml version="1.0" encoding="UTF-8" ?>
// <svg xmlns="http://www.w3.org/2000/svg" version="1.1">
// <rect x="25" y="25" width="200" height="200" fill="lime" stroke-width="4" stroke="pink" />
// <circle cx="125" cy="125" r="75" fill="orange" />
// <polyline points="50,150 50,200 200,200 200,100" stroke="red" stroke-width="4" fill="none" />
// <line x1="50" y1="50" x2="200" y2="200" stroke="blue" stroke-width="4" />
// </svg>


use std::fs::File;
use std::io::Write;
use std::error::Error;


// utility for creating very basic colors for SVG writing
pub enum Color {
    Red, Blue, Green, Yellow, Black, White, Grey, None
}

// convert a Color enum to a SVG string
pub fn color_to_string(c: &Color) -> String {
    return match *c {
        Color::Red    =>    "red".to_owned(),
        Color::None   =>   "none".to_owned(),
        Color::Blue   =>   "blue".to_owned(),
        Color::Grey   =>   "grey".to_owned(),
        Color::Green  =>  "green".to_owned(),
        Color::Black  =>  "black".to_owned(),
        Color::White  =>  "white".to_owned(),
        Color::Yellow => "yellow".to_owned(),
    }
}

// any SVG object we want to store in our SVG document should have a to_string() func
pub trait SVGObject {
    fn to_string(&self) -> String;
}

pub struct SVG {
    pub width:                       u64,
    pub height:                      u64,
    pub view_width:                  u64,
    pub view_height:                 u64,
    pub objects:     Vec<Box<SVGObject>>,
}

pub struct SVGLine {
    pub x1:       u64,
    pub y1:       u64,
    pub x2:       u64,
    pub y2:       u64,
    pub stroke:   u64,
    pub color:  Color,
}

pub struct SVGRect {
    pub x:      u64,
    pub y:      u64,
    pub w:      u64,
    pub h:      u64,
    pub fill: Color,
}

pub struct SVGCircle {
    pub cx:     u64,
    pub cy:     u64,
    pub radius: u64,
}

pub struct SVGVertex {
    pub x: u64,
    pub y: u64,
}

pub struct SVGPoly {
    pub color:             Color,
    pub stroke:              u64,
    pub vertices: Vec<SVGVertex>,
}


// implementations

impl SVGLine {
    pub fn new(x1: u64, y1: u64, x2: u64, y2: u64, w: u64, color: Color) -> SVGLine {
        SVGLine{x1: x1, y1: y1, x2: x2, y2: y2, stroke: w, color: color}
    }
}

// <line x1="50" y1="50" x2="200" y2="200" stroke="blue" stroke-width="4" />
impl SVGObject for SVGLine {
    fn to_string(&self) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" />",
            self.x1, self.y1, self.x2, self.y2,
            color_to_string(&self.color), self.stroke,
        )
    } 
}


// <rect x="25" y="25" width="200" height="200" fill="lime" stroke-width="4" stroke="pink" />
impl SVGRect {
    pub fn new(x: u64, y: u64, w: u64, h: u64, fill: Color) -> SVGRect {
        SVGRect{x: x, y: y, w: w, h: h, fill: fill,}
    }
}

impl SVGObject for SVGRect {
    fn to_string(&self) -> String {
        format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />",
            self.x, self.y, self.w, self.h, color_to_string(&self.fill),
        )
    }
}


// <circle cx="125" cy="125" r="75" fill="orange" />
impl SVGCircle {
    pub fn new(cx: u64, cy: u64, r: u64) -> SVGCircle {
        SVGCircle{cx: cx, cy: cy, radius: r,}
    }
}

impl SVGObject for SVGCircle {
    fn to_string(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" />",
            self.cx, self.cy, self.radius, "none"
        )
    }
}


impl SVGVertex {
    pub fn new(x: u64, y: u64) -> SVGVertex {
        SVGVertex{x: x, y: y}
    }

    pub fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

impl SVGPoly {
    pub fn new(c: Color, stroke: u64) -> SVGPoly {
        let v : Vec<SVGVertex> = Vec::new();
        SVGPoly{
            color: c,
            stroke: stroke,
            vertices: v,
        }
    }

    pub fn addv(&mut self, x: u64, y: u64) {
        self.vertices.push(SVGVertex::new(x,y));
    }
}

impl SVGObject for SVGPoly {
    fn to_string(&self) -> String {
        String::from("not implemented")
    }
}


// implementation for the SVG container
// <svg xmlns="http://www.w3.org/2000/svg" version="1.1">
impl SVG {

    // craft a new SVG and set the width and height at creation time
    pub fn new(w: u64, h: u64, vx: u64, vy: u64) -> SVG {
        return SVG{
            width:                w,
            height:               h,
            view_width:          vx,
            view_height:         vy,
            objects:     Vec::new(),
        }
    }


    // add an object to the container as long as it implements the needed trait
    pub fn add_object(&mut self, sobj: Box<SVGObject>) -> usize {
        self.objects.push(sobj);
        return self.objects.len();
    }


    // convert an SVG object to file format
    pub fn to_file(&mut self, fname: &str) -> Result<u8, &str> {
        let head = format!(
            "<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
            self.width, self.height, self.view_width, self.view_height,
        );
        let tail = String::from("</svg>");
        let mut buf : Vec<String> = Vec::new();

        buf.push(head);
        for obj in &self.objects {
            buf.push(obj.to_string().to_owned());
        }
        buf.push(tail);

        // open the file for writing
        let mut f = match File::create(fname) {
            Ok(new_file) => new_file,
            Err(why)     => panic!("Couldn't create '{:?}': {}", fname, why.description()),
        };

        for stringthing in buf {
            match f.write(stringthing.as_ref()) {
                Ok(_) => {},
                _     => panic!("Failed to write bytes"),
            };
        }
        return Ok(0);
    }
}


// tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_create_svg() {
        use svg::*;
        
        let mut s = SVG::new(1024, 1024, 1024, 1024);

        let rect  = SVGRect::new(0, 0, 1024, 1024, Color::White);
        let line  = SVGLine::new(0, 0, 1024, 1024, 5, Color::Black);
        let line2 = SVGLine::new(1024, 0, 0, 1024, 10, Color::Black);

        s.add_object(Box::new(rect));
        s.add_object(Box::new(line));
        s.add_object(Box::new(line2));

        s.to_file("test.svg");
    }
}
