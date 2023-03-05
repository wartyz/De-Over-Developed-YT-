use rand::prelude::{SliceRandom, ThreadRng};
use rand::thread_rng;
use self::piece::{Piece, Kind as PieceKind};

mod piece;

pub struct Engine {
    board: Board,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::blank(),
            bag: Vec::new(),
            rng: thread_rng(),
        }
    }

    fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        // Pone las piezas en la bolsa
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        // Baraja la bolsa
        self.bag.shuffle(&mut self.rng)
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
