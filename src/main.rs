use core::panic;
use std::borrow::BorrowMut;

use image::{ImageBuffer, Rgba, RgbaImage, imageops, io::Reader as ImageReader};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point{
        Point {
            x, y
        }
    }
}

// Struct that respresents equation of a straight line y = mx + b
// Can be invoked 
#[derive(Debug, Clone, Copy)]
struct FnLine {
    slope: f32,
    offset: f32,
}

impl FnLine {
    pub fn new(slope: f32, offset: f32) -> FnLine {
        FnLine {
            slope,
            offset
        }
    }

    pub fn from_points(p1: Point, p2: Point) -> FnLine {
        let slope =  (p2.y - p1.y) / (p2.x - p1.x);
        FnLine {
            slope: slope,
            offset: p1.y - slope * p1.x
        }
    }
    
    pub fn eval(&self, x: f32) -> f32 {
        self.slope * x + self.offset 
    }

    #[inline]
    // Check if a point is below a given line
    pub fn below(&self, p: Point) -> bool {
        p.y <= self.eval(p.x)
    }

    #[inline]
    pub fn above(&self, p: Point) -> bool {
        !self.below(p)

    }
}

#[derive(PartialEq)]
enum HexType {
    Pointy, Flat
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    if args.len() < 3 {
        println!("Usage: hexagen <hex-radius> <output-path> <hex-type>\nHex type is either 'Pointy' or 'Flat'");
        return;
    }

    // Input 
    let radius = &args[0].parse::<f32>().unwrap();
    let fill_color = Rgba([255, 255, 255, 255]);
    let output_file_name = &args[1];
    let hex_type = match args[2].as_str() {
        "Flat" => HexType::Flat,
        "Pointy" => HexType::Pointy,
        _ => panic!("Hex type should be either 'Pointy' or 'Flat'")
    };

    // Calculations
    let diameter = 2.0 * radius;
    let side = diameter * 0.5; 
    let inradius = (radius.powf(2.0) - 1.0/2.0 * side).sqrt();
    let twice_inradius = 2.0 * inradius;

    let image_height = diameter;
    let image_width = twice_inradius;

    let length_corner_triangle_vertical = (twice_inradius - side) / 2.0; 
    let length_corner_triangle_horizontal = inradius;

    // Top-left triangle
    let (tr1_p1, tr1_p2) = (
        Point::new(0.0, length_corner_triangle_vertical),
        Point::new(length_corner_triangle_horizontal, 0.0));
    let tr1_line = FnLine::from_points(tr1_p1, tr1_p2);

    // Top right triangle
    let (tr2_p1, tr2_p2) = (
        Point::new(length_corner_triangle_horizontal, 0.0),
        Point::new(2.0 * inradius, length_corner_triangle_vertical));
    let tr2_line = FnLine::from_points(tr2_p1, tr2_p2);

    // Bottom left triangle
    let (tr3_p1, tr3_p2) = (
        Point::new(0.0, length_corner_triangle_vertical + side),
        Point::new(length_corner_triangle_horizontal, image_height));
    let tr3_line = FnLine::from_points(tr3_p1, tr3_p2);

    // Bottom right triangle
    let (tr4_p1, tr4_p2) = (   
        Point::new(length_corner_triangle_horizontal, image_height),
        Point::new(2.0 * inradius, length_corner_triangle_vertical + side));
    let tr4_line = FnLine::from_points(tr4_p1, tr4_p2);

    // Check if any given point is inside any of the defined triangles, fill with alpha if true
    // Not the best implementation, but will work
    let should_be_alpha = |p: Point| -> bool {
        tr1_line.below(p) || tr2_line.below(p) || tr3_line.above(p) || tr4_line.above(p)
    };

    println!("Image size: {0}/{1}", image_width as u32, image_height as u32);
    // Create image
    let mut img_buff = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(image_width as u32, image_height as u32);

    for px in 0..image_width as u32 {
        for py in 0..image_height as u32 {
            let pixel = match should_be_alpha(Point::new(px as f32, py as f32)) {
                true => Rgba([255, 255, 255, 0]),
                false => fill_color,
            };
            img_buff.put_pixel(px, py, pixel);
        }
    }
    
    // By default a pointy hexagon is generated, rotate by 90 degrees if a flat one is requested
    if hex_type == HexType::Flat {
        img_buff = imageops::rotate90(&img_buff);
    } 

    // Output 
    img_buff.save(output_file_name).unwrap();
}