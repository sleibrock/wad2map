// sidedef.rs

pub struct SideDef {
    pub x_offset: i16,
    pub y_offset: i16,
    pub upper_tex: String,
    pub lower_tex: String,
    pub middle_tex: String,
    pub sector: u16,
}


impl SideDef {
    pub fn new(dat: &[u8]) -> SideDef {
        if dat.len() != SIDEDEF_WIDTH {
            panic!("SideDef given {} bytes, needs {}", dat.len(), SIDEDEF_WIDTH);
        }

        // calculate string lengths so no NUL bytes are included
        let mut zero1 : usize = 11;
        let mut zero2 : usize = 19;
        while dat[zero1] == 0 {
            zero1 -= 1;
        }
        while dat[zero2] == 0 {
            zero2 -= 1;
        }

        SideDef{
            
        }
    }
}
