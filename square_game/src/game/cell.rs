enum Side {
    Up,
    Down,
    Right,
    Left,
}

struct Cell {
    up: bool,
    down: bool,
    right: bool,
    left: bool,

    // If none the cell is not closed
    // if i8 - the player that takes the cell
    closed: Option<u8>,
}

imp Cell {
    pub fn default() -> Self {
        Self {
            up: false,
            down: false,
            right: false,
            left: false,
            
            closed: None,
        }
    }

    pub fn set_side(mut& self, side: Side) {
        match side {
            Side::Up => self.Up = true,
            Side::Down => self.Down = true,
            Side::Right => self.Right = true,
            Side::Left => self.Left = true,
        }
    }
}