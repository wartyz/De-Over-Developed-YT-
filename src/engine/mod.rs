pub struct Engine {
    board: Board,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::blank()
        }
    }
}

struct Board([bool; Self::SIZE]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn blank() -> Self {
        Self([false; Self::SIZE])
    }
}