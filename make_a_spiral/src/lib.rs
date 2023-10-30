enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn switch(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn step(&self, point: &Point) -> Point {
        match self {
            Direction::Up => Point(point.0, point.1 - 1),
            Direction::Right => Point(point.0 + 1, point.1),
            Direction::Down => Point(point.0, point.1 + 1),
            Direction::Left => Point(point.0 - 1, point.1),
        }
    }
}

struct Point(usize, usize);

struct Walls {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

impl Walls {
    fn new(size: usize) -> Walls {
        Walls {
            min_x: 0,
            min_y: 0,
            max_x: size - 1,
            max_y: size - 1,
        }
    }

    fn move_wall(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => self.min_y += 2,
            Direction::Down => self.max_x -= 2,
            Direction::Left => self.max_y -= 2,
            Direction::Up => self.min_x += 2,
        };
    }

    fn is_end(&self, direction: &Direction, point: &Point) -> bool {
        match direction {
            Direction::Up => point.1 == self.min_y,
            Direction::Right => point.0 == self.max_x,
            Direction::Down => point.1 == self.max_y,
            Direction::Left => point.0 == self.min_x,
        }
    }

    fn continue_spiral(&self) -> bool {
        self.min_x <= self.max_x && self.min_y <= self.max_y
    }
}

pub fn spiralize(size: usize) -> Vec<Vec<i8>> {
    // Create an empty array of size x size
    let mut spiral: Vec<Vec<i8>> = vec![vec![0; size]; size];

    // Check if minimum size is 5 and return if not
    if size < 5 {
        return spiral;
    }

    let mut direction = Direction::Right;
    let mut p: Point = Point(0, 0);
    let mut walls = Walls::new(size);

    // Crawl the spiral
    while walls.continue_spiral() {
        spiral[p.1][p.0] = 1;

        // Check if at a wall
        if walls.is_end(&direction, &p) {
            walls.move_wall(&direction);
            direction = direction.switch();
        }

        // Move
        p = direction.step(&p);
    }

    // Fix the last step, based on the size
    if size % 2 == 0 {
        spiral[p.1][p.0] = 1;
    }

    spiral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}
