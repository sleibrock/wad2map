// level.rs

use utils::*;
use doom::linedef::*;
use doom::vertex::*;
use doom::constants::{DOOM_LINEDEF_WIDTH, HEXEN_LINEDEF_WIDTH, VERTEX_WIDTH};

/// A Level is a collection of all types of Lump group categories into one piece.
/// A Level here has two lists, a VERTEXES and LINEDEFS list.
/// TODO: add support for THINGS
pub struct Level {
    pub name:     String,
    pub vertices: Vec<Vertex>,
    pub linedefs: Vec<LineDef>,
}

impl Level {
    pub fn new(name: &String, vert_raw: &[u8], ld_raw: &[u8], is_hexen: bool) -> Level {
        // create vectors of items we need to store
        let mut vertices : Vec<Vertex> = Vec::new();
        let mut linedefs : Vec<LineDef> = Vec::new();

        // determine the width we will be using for LINEDEF scanning
        let ld_width : usize = match is_hexen {
            true => HEXEN_LINEDEF_WIDTH,
            false => DOOM_LINEDEF_WIDTH,
        };

        let mut offset : usize = 0;
        while offset < vert_raw.len() {
            vertices.push(Vertex::new(&vert_raw[packet_range(offset, VERTEX_WIDTH)]));
            offset += VERTEX_WIDTH;
        }

        offset = 0;
        while offset < ld_raw.len() {
            linedefs.push(LineDef::new(
                is_hexen,
                &ld_raw[packet_range(offset, ld_width)],
            ));
            offset += ld_width;
        }

        Level{
            name:     name.to_owned(),
            vertices: vertices,
            linedefs: linedefs,
        }
    }

    pub fn print(&self) {
        println!("Level name: {}", self.name);
        println!("Vertices: {}", self.vertices.len());
        println!("Linedefs: {}", self.linedefs.len());
    }
}

// end
