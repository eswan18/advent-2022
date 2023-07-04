#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn build_from_tuple(tuple: (i32, i32)) -> Point {
        Point { x: tuple.0, y: tuple.1 }
    }
}