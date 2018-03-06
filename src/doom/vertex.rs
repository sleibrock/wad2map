// vertex.rs

use utils::u8_to_i16;
use doom::constants::VERTEX_WIDTH;


/// A Vertex is a 4-byte slice of data representing a vertex in 2D space.
/// Every other object in Doom files will reference a Vertex.
/// Vertices are stored in Signed Integer format as 16-bit values
pub struct Vertex {
    pub x: i16,
    pub y: i16,
}


impl Vertex {
    pub fn new(dat: &[u8]) -> Vertex {
        if dat.len() != VERTEX_WIDTH {
            panic!(format!("Vertex not given {} bytes", VERTEX_WIDTH));
        }

        Vertex{ 
            x: u8_to_i16(dat[0], dat[1]),
            y: u8_to_i16(dat[2], dat[3]),
        }
    }

    // debugging purposes
    pub fn print(&self) {
        println!("Vertex({}, {})", self.x, self.y);
    }
}

// end
