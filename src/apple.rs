use point::Point;

pub struct Apple {
    pub location: Point,
}

impl Apple {
    pub fn new(x: i32, y: i32) -> Apple {
        Apple { location: Point { x: x, y: y } }
    }
}