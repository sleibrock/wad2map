/// SVG file API
/// The SVG struct holds shapes and can render to file
///
/// Example file:
/// <?xml version="1.0" encoding="UTF-8" ?>
/// <svg xmlns="http://www.w3.org/2000/svg" version="1.1">
/// <rect x="25" y="25" width="200" height="200" fill="lime" stroke-width="4" stroke="pink" />
/// <circle cx="125" cy="125" r="75" fill="orange" />
/// <polyline points="50,150 50,200 200,200 200,100" stroke="red" stroke-width="4" fill="none" />
/// <line x1="50" y1="50" x2="200" y2="200" stroke="blue" stroke-width="4" />
/// </svg>
/// 


pub enum Color {
    Red, Blue, Green, Yellow, Black, White, Grey,
}

pub trait SVGObject {
    fn to_string(&self) -> String;
}

pub struct SVG {
    pub objects: Vec<Box<SVGObject>>,
}

pub struct SVGLine {
    pub    x1:   u64,
    pub    y1:   u64,
    pub    x2:   u64,
    pub    y2:   u64,
    pub color: Color,
}

pub struct SVGRect {
}

pub struct SVGCircle {
}


// implementations (do traits first)

/// <line x1="50" y1="50" x2="200" y2="200" stroke="blue" stroke-width="4" />
impl SVGObject for SVGLine {
    fn to_string(&self) -> String {
        return String::from("");
        return format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" />",
            self.x1, self.y1, self.x2, self.y2,
            match self.color {
                _ => "black",
            },
            2,
        );
    } 
}



// implementation for the SVG container


impl SVG {
    pub fn new() -> SVG {
        return SVG{
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, sobj: Box<SVGObject>) -> usize {
        self.objects.push(sobj);
        return self.objects.len();
    }


    pub fn to_file(&self, fname: &str) -> Result<u8, &str> {


        return Ok(0);
    }
}
