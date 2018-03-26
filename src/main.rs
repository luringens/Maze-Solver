extern crate clap;
extern crate image;
extern crate pathfinding;

use clap::{Arg, App};
use image::{ImageBuffer, Rgb};
use pathfinding::astar::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(u32, u32);

impl Pos {
  fn distance(&self, other: &Pos) -> usize {
    ((self.0 - other.0) + (self.1 - other.1)) as usize
  }

  fn neighbours(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<(Pos, usize)> {
    let &Pos(x, y) = self;
    let (w, h) = img.dimensions();
    vec![Pos(x+1,y), Pos(x-1,y), Pos(x,y+1), Pos(x,y-1)]
        .into_iter()
        // Assuming underflow causes numbers larger than h & w
        .filter(|p| p.0 < w && p.1 < h)
        .filter(|p| img.get_pixel(p.0, p.1).data[0] > 122)
        .map(|p| (p, 1))
        .collect()
  }
}

fn main() {
    let matches = App::new("Maze Solver")
        .version("1.0")
        .author("Stian S. <soltvedt.stian@gmail.com>")
        .about("Solves a maze provided with an image file.")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    
    let filepath = matches.value_of("INPUT").unwrap();

    let mut rgb = image::open(filepath).expect("Failed to open image")
        .grayscale().adjust_contrast(std::f32::MAX)
        .to_rgb();

    static GOAL: Pos = Pos(0, 2047);
    let result = astar(
        &Pos(0, 1990),
        |p| p.neighbours(&rgb),
        |p| p.distance(&GOAL) / 3,
        |p| *p == GOAL
    );

    if let Some((path, _)) = result {
        for p in &path {
            rgb.put_pixel(p.0, p.1, Rgb {data: [255, 0, 0]});
        }

        println!("Path length: {}", path.len());

        rgb.save(format!("{}{}", filepath, "_solved.png"))
            .expect("Failed to save image");
    } else {
        println!("No path found");
    }
}
