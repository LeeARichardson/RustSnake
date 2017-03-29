use std::collections::LinkedList;

use point::Point;

extern crate num;

const GAME_FIELD_WIDTH: usize = 20;
const GAME_FIELD_HEIGHT: usize = 20;
const POINTS_SIZE: usize = (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize;

pub struct Snake <'a> {
    points: &'a [Point; POINTS_SIZE],
    pub head: &'a Point,
    pub body: LinkedList<&'a Point>,
    pub direction: Point,
    pub last_removed_body_position: Option<&'a Point>,
}

fn get_point(points: &[Point; POINTS_SIZE], x: usize, y: usize) -> &Point {
    let clamped_x = num::clamp(x, 0, GAME_FIELD_WIDTH);
    let clamped_y = num::clamp(y, 0, GAME_FIELD_HEIGHT);

    &points[(clamped_x * clamped_y) + clamped_x]
}

impl<'a> Snake<'a> {
    pub fn new(points: &'a [Point; POINTS_SIZE], x: i32, y: i32) -> Snake<'a> {
        Snake {
            points: points,
            head: get_point(points, x as usize, y as usize),
            body: LinkedList::new(),
            direction: Point::new(1, 0), // Right
            last_removed_body_position: None,
        }
    }

    pub fn movement(&mut self) {
        self.body.push_front(self.head);
        self.head = get_point(&self.points,
                              (self.head.x + self.direction.x) as usize,
                              (self.head.y + self.direction.y) as usize);

        self.last_removed_body_position = self.body.pop_back();
    }

    pub fn grow(&mut self) {
        self.body.push_back(self.last_removed_body_position.clone().unwrap());
    }
}

#[test]
fn test(){
    let ref point = Point::new(1,2);
     assert_eq!(point.x,1);
}