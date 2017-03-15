use std::collections::LinkedList;

use point::Point;

pub struct Snake {
    pub head: Point,
    pub body: LinkedList<Point>,
    pub direction: Point,
    pub last_removed_body_position: Option<Point>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        Snake {
            head: Point::new(x, y),
            body: LinkedList::new(),
            direction: Point::new(1, 0), // Right
            last_removed_body_position: None,
        }
    }

    pub fn movement(&mut self) {
        self.body.push_front(self.head.clone());
        self.head = Point::new(self.head.x + self.direction.x,
                               self.head.y + self.direction.y);

        self.last_removed_body_position = self.body.pop_back();
    }

    pub fn grow(&mut self) {
        self.body.push_back(self.last_removed_body_position.clone().unwrap());
    }
}