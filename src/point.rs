use std::fmt::{Display, Formatter, Result};

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub is_intersection: bool,
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let symbol = if self.is_intersection { "X" } else { "O" };
        // Making sure that one and two digit values have same
        // String size.
        if self.x > 9 && self.y > 9 {
            write!(f, "{}({},{})", symbol, self.x, self.y)
        } else if self.x > 9 && self.y < 10 {
            write!(f, "{}({},0{})", symbol, self.x, self.y)
        } else if self.y > 9 && self.x < 10 {
            write!(f, "{}(0{},{})", symbol, self.x, self.y)
        } else {
            write!(f, "{}(0{},0{})", symbol, self.x, self.y)
        }
    }
}