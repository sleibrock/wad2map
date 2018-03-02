// constants.rs

/// constants that describe byte widths for structs
/// Conglomerating them into one single file makes it easier to use
/// across different struct definitions
pub const  LUMP_WIDTH          : usize = 16;

pub const  SEG_WIDTH           : usize = 12;
pub const  HEADER_WIDTH        : usize = 12;
pub const  VERTEX_WIDTH        : usize =  4;
pub const  SECTOR_WIDTH        : usize = 26;
pub const  SSECTOR_WIDTH       : usize =  4;
pub const  SIDEDEF_WIDTH       : usize = 30;
pub const  DOOM_LINEDEF_WIDTH  : usize = 14;
pub const  HEXEN_LINEDEF_WIDTH : usize = 16;

/// These numbers are used in determining the type of Wad that we are given.
/// If a file does not match these two numbers, then it is not a proper Wad
pub const IWAD_NUMBER : u32 = 1145132873;
pub const PWAD_NUMBER : u32 = 1145132880;

// end
