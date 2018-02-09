// A Wad2Map test program

// all numbers are in Little Endian format

use std::env;
use std::process::exit;
use std::mem::transmute;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str;


pub struct WadInfo {
    pub identification: u32,
    pub numlumps:       u32,
    pub infotableofs:   u32,
}


impl WadInfo {
    pub fn new(datum: [u8; 12]) -> WadInfo {
        return WadInfo{
            identification: 0,
            numlumps:       0,
            infotableofs:   0,
        };
    }


    // debugging purposes
    pub fn print_info(&self) {
        print!("Type of file: ");
        match self.identification {
            100 => { println!("IWAD"); },
            200 => { println!("PWAD"); },
            _   => { println!("NULL"); },
        }
        
    }
}


pub struct FileLump {
    pub filepos:        u32,
    pub size:           u32,
    pub name:        String,
}


pub struct LineDef {
    pub a:          u16,
    pub b:          u16,
    pub left_side:  i16,
    pub right_side: i16,
}

impl LineDef {
    pub fn is_one_sided(&self) -> bool {
        return self.left_side == 0 || return self.right_side == 0;
    }
}


pub struct SVG {
    pub name:         [u8; 8],
    pub linebuf: Vec<LineDef>,
}


impl SVG {
    pub fn new(name: [u8; 8]) -> SVG {
        return SVG{
            name: name,
            linebuf: Vec::new(),
        }
    }

    pub fn to_file(&self) {
    }
}




/// actual program logic goes below here i guess

fn open_and_read_wad(fname: &str) {
    println!("Opening up {}...", fname);

    
    let mut f = File::open(fname).expect("File not found");
    let mut buffer = [0; 16];

    for x in 0..10 {
        match f.read(&mut buffer) {
            Ok(x) => {
                println!("{}", String::from_utf8_lossy(&buffer));
            },
            _ => { panic!("HELP"); }
        }
    }


}


fn main() {
    let mut arg_iter = env::args();
    arg_iter.next();

    if arg_iter.len() == 0 {
        println!("Args list empty!");
        exit(-1);
    }

    for arg in arg_iter {
        open_and_read_wad(format!("{}", arg).as_str());
    }


    exit(0);
}
