use std::fmt::{Display, Formatter, Result};

fn main() {
    // println!("Hello, world!");
    // println!("I Edited this code and pushing to branch-test");
    // println!("Testing 2nd Merge");
    
    // Testing
    let grid = generate_grid();
    print!("{}", grid);
}

// Generate grid
struct Point {
    x: i32,
    y: i32,
    is_intersection: bool,
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.is_intersection)
    }
}

struct Grid {
    points: Vec<Point>,
}
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for point in &self.points {
            writeln!(f, "{}", point)?; // Writes each point on a new line
        }
        Ok(())
    }
}

fn generate_grid() -> Grid {
    let mut points = vec![];
    let height = 2;
    let width = 2;

    for i in 0..height {
        for j in 0..width {
            points.push(Point{x: i, y: j, is_intersection: false});
        }
    }

    Grid{points: points}
}
