pub struct Point {
    pub x: usize,
    pub y: usize
}

impl PartialEq<(usize, usize)> for Point {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
}