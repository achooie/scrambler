use std::slice::Iter;

enum Color {
    White,
    Yellow,
    Green,
    Blue,
    Orange,
    Red,
}

impl Color {
    fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 6] =
            [Color::White, Color::Yellow, Color::Green, Color::Blue, Color::Orange, Color::Red];

        COLORS.iter()
    }
}

struct Face {
    tiles: Vec<&'static Color>,
}

struct Cube {
    faces: Vec<Face>,
}

impl Cube {
    fn new() -> Cube {
        Cube { faces: Color::iterator().map(|c| Face { tiles: vec![c; 8] }).collect::<Vec<Face>>() }
    }
}

fn main() {
    println!("Hello, world!");
}
