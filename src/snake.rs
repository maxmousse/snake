use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        
        let body = (0..length)
            .into_iter()
            .map(|i| start.transform(opposite, i))
            .collect();
            
        Self {
            body,
            direction,
            digesting: false,
        }
    }
    
    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone()
    }
    
    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone()
    }
    
    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }
    
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    
    pub fn set_digesting(&mut self, digesting: bool) {
        self.digesting = digesting;
    }
    
    pub fn contains_point(&self, point: &Point) -> bool {
        self.body.contains(point)
    }
    
    pub fn move_ahead(&mut self) {
        self.body.insert(0, self.body.first().unwrap().transform(self.direction, 1));

        if self.digesting {
            self.digesting = false;
        } else {
            self.body.remove(self.body.len() -1);
        }
    }
}