use std::cmp::{max};
use log::*;
use std::slice::Iter;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West
}


impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction;  4] = [North, South, East, West];
        DIRECTIONS.into_iter()
    }
}

impl Location {
    pub fn go_one(&self, dir: Direction) -> Location {
        self.go(dir, 1)
    }

    pub fn go(&self, dir: Direction, distance: i32) -> Location {
        let r = match dir {
            Direction::North => Location{x: self.x, y: self.y - distance},
            Direction::South => Location{x: self.x, y: self.y + distance},
            Direction::East => Location{x: self.x + distance, y: self.y},
            Direction::West => Location{x: self.x - distance, y: self.y},
        };
        r
    }

    pub fn manhattan(&self, other: Location) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn xy(x: i32, y: i32) -> Location {
    Location{x, y}
}

/// An auto-sizing 2D grid of data
#[derive(Clone)]
pub struct Grid<T: Clone> {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,

    data: Vec<Option<T>>,
    default: Option<T>,
}

impl<T: Clone> Grid<T> {
    pub fn empty() -> Grid<T> {
        Self::empty_with_default(None)
    }
    
    pub fn empty_with_default(default: Option<T>) -> Grid<T> {
        Grid{top:0, left:0, width: 0, height: 0, default, data: Vec::<Option<T>>::new()}
    }
    pub fn new(left: i32, top: i32, width: i32, height: i32, default: Option<T>) -> Grid<T> {
        Grid{top, left, width, height, default: default.clone(), data: vec![default.clone(); (width*height) as usize]}
    }

    pub fn set(&mut self, loc: &Location, value: Option<T>) {
        self.check_and_grow(loc);
        let offset = (loc.y - self.top) * self.width + (loc.x - self.left);
        self.data[offset as usize] = value;
    }

    pub fn iter(&mut self) -> GridIterator<T> {
        GridIterator{x: self.left, y: self.top, grid: self}
    }

    pub fn get(&mut self, loc: &Location) -> Option<T> {
        self.check_and_grow(loc);
        // if loc.x < self.left || loc.x >= self.width + self.left || loc.y < self.top || loc.y >= self.height + self.top {
        //     debug!("Out of range get");
        //     return None;
        // }
        let offset = (loc.y - self.top) * self.width + (loc.x - self.left);
        self.data[offset as usize].clone()
    }

    fn resize(&mut self, left: i32, top: i32, width: i32, height: i32) {
        debug!("Resizing to {}, {}, {}, {}", left, top, width, height);
        let mut old = self.clone();
        self.top = top;
        self.left = left;
        self.width = width;
        self.height = height;
        self.data = vec![self.default.clone(); (width*height) as usize];

        for x in old.left..old.left+old.width {
            for y in old.top..old.top+old.height {
                self.set(&xy(x, y), old.get(&xy(x, y)));
            }
        }
    }

    fn check_and_grow(&mut self, loc: &Location) {

        let mut resize = false;
        let mut new_left = self.left;
        let mut new_top = self.top;
        let mut new_width = self.width;
        let mut new_height = self.height;
        let min_adjust_size = max(max(self.width, self.height), 128);
        if loc.x < self.left {
            let adjust_size = max(min_adjust_size, self.left - loc.x);
            new_left -= adjust_size;
            new_width += adjust_size;
            resize = true;
        } else if loc.x >= self.left + self.width {
            let adjust_size = max(min_adjust_size, loc.x - self.left - self.width + 1);
            new_width += adjust_size;
            resize = true;
        }
        if loc.y < self.top {
            let adjust_size = max(min_adjust_size, self.top - loc.y);
            new_top -= adjust_size;
            new_height += adjust_size;
            resize = true;
        } else if loc.y >= self.top + self.height {
            let adjust_size = max(min_adjust_size, loc.y - self.top - self.height + 1);
            new_height += adjust_size;
            resize = true;
        }
        if resize {
            self.resize(new_left, new_top, new_width, new_height);
        }
    }
}

pub struct GridIterator<'a, T: Clone> {
    x: i32,
    y: i32,
    grid: &'a mut Grid<T>,
}

impl<'a, T: Clone> Iterator for GridIterator<'a, T> {
    type Item = (Location, Option<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.grid.width + self.grid.left || self.y >= self.grid.height + self.grid.top {
            return None
        }
        let item = self.grid.get(&xy(self.x, self.y));
        self.x += 1;
        if self.x >= self.grid.width + self.grid.left {
            self.x = self.grid.left;
            self.y += 1;
        }
        Some((xy(self.x, self.y), item))
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::*;
    use simple_logger;

    #[derive(Clone, Debug, PartialEq)]
    struct Node {
        a: u32,
    }

    #[test]
    fn test_empty_grid() {
        let _ = simple_logger::init();

        let mut grid = Grid::<Node>::empty();
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);

        grid.set(&xy(-1, 2), Some(Node{a: 10}));
        grid.set(&xy(2, 2), Some(Node{a: 11}));
        grid.set(&xy(4, -4), Some(Node{a: 12}));

        assert_eq!(grid.get(&xy(-1, 2)).unwrap(), (Node{a: 10}));
        assert_eq!(grid.get(&xy(2, 2)).unwrap(), (Node{a: 11}));
        assert_eq!(grid.get(&xy(4, -4)).unwrap(), (Node{a: 12}));
        assert!(grid.get(&xy(0, 0)).is_none());
    }

    #[test]
    fn test_init_grid() {
        let mut grid = Grid::<Node>::new(0, 0, 10, 10, Some(Node{a:123}));
        assert_eq!(grid.get(&xy(0, 1)).unwrap(), Node{a: 123});
    }

    #[test]
    fn test_location() {
        assert_eq!(xy(2, 3).go_one(Direction::East), xy(3, 3));
        assert_eq!(xy(2, 3).go_one(Direction::West), xy(1, 3));
        assert_eq!(xy(2, 3).go_one(Direction::North), xy(2, 2));
        assert_eq!(xy(2, 3).go_one(Direction::South), xy(2, 4));
    }
}