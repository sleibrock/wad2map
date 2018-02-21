use utils::*;

use doom::constants::{VERTEX_WIDTH};


pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

/// A Vertex is a 4-byte slice of data representing a vertex in 2D space.
/// Every other object in Doom files will reference a Vertex.
/// Vertices are stored in Signed Integer format as 16-bit values
impl Vertex {
    pub fn new(dat: &[u8]) -> Vertex {
        if dat.len() != VERTEX_WIDTH {
            panic!(format!("Vertex not given {} bytes", VERTEX_WIDTH));
        }
        Vertex{
            x: u8_to_u16(dat[0], dat[1]) as i16,
            y: u8_to_u16(dat[2], dat[3]) as i16,
        }
    }

    pub fn print(&self) {
        println!("Vertex({}, {})", self.x, self.y);
    }
}

