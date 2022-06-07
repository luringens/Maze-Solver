#[macro_use]
extern crate clap;
extern crate image;
extern crate pathfinding;

use clap::{App, Arg};
use image::{ImageBuffer, Rgb};
use pathfinding::astar::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        ((self.0 - other.0) + (self.1 - other.1)) as usize
    }

    fn neighbours(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let (w, h) = (img.width() as i32, img.height() as i32);
        vec![Pos(x + 1, y), Pos(x - 1, y), Pos(x, y + 1), Pos(x, y - 1)]
            .into_iter()
            .filter(|p| p.0 < w && p.1 < h && p.0 >= 0 && p.1 >= 0)
            .filter(|p| img.get_pixel(p.0 as u32, p.1 as u32).0[0] > 122)
            .map(|p| (p, 1))
            .collect()
    }
}

fn main() {
    let matches = App::new("Maze Solver")
        .version("1.0")
        .author("Stian S. <soltvedt.stian@gmail.com>")
        .about("Solves a maze provided by an image file.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("x1")
                .long("startx")
                .help("Sets the x coordinate of the start position")
                .takes_value(true)
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("y1")
                .long("starty")
                .short("sy")
                .help("Sets the y coordinate of the start position")
                .takes_value(true)
                .required(true)
                .index(3),
        )
        .arg(
            Arg::with_name("x2")
                .long("goalx")
                .short("gx")
                .help("Sets the x coordinate of the goal position")
                .takes_value(true)
                .required(true)
                .index(4),
        )
        .arg(
            Arg::with_name("y2")
                .long("goaly")
                .short("gy")
                .help("Sets the y coordinate of the goal position")
                .takes_value(true)
                .required(true)
                .index(5),
        )
        .get_matches();

    let filepath = matches
        .value_of("INPUT")
        .expect("Failed to parse input file path.");

    let x1 =
        value_t!(matches, "x1", u32).expect("Failed to parse input for x1 as a positive integer.");
    let y1 =
        value_t!(matches, "y1", u32).expect("Failed to parse input for y1 as a positive integer.");
    let x2 =
        value_t!(matches, "x2", u32).expect("Failed to parse input for x2 as a positive integer.");
    let y2 =
        value_t!(matches, "y2", u32).expect("Failed to parse input for y2 as a positive integer.");

    let mut rgb = image::open(filepath)
        .expect("Failed to open image")
        .grayscale()
        .adjust_contrast(std::f32::MAX)
        .to_rgb8();

    let goal: Pos = Pos(x2 as i32, y2 as i32);
    let result = astar(
        &Pos(x1 as i32, y1 as i32),
        |p| p.neighbours(&rgb),
        |p| p.distance(&goal) / 3,
        |p| *p == goal,
    );

    if let Some((path, _)) = result {
        for p in &path {
            rgb.put_pixel(p.0 as u32, p.1 as u32, Rgb([255, 0, 0]));
        }

        println!("Path length: {}", path.len());

        rgb.save(format!("{}{}", filepath, "_solved.png"))
            .expect("Failed to save image");
    } else {
        println!("No path found");
    }
}
