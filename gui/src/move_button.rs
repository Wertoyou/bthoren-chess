use chackad::piece::PieceType;

pub struct MoveButton {
    from: (usize, usize),
    to: (usize, usize, PieceType),
}

impl MoveButton {
    pub fn new(from: (usize, usize), to: (usize, usize, PieceType)) -> Self {
        Self { from, to }
    }

    pub fn from(&self) -> (usize, usize) {
        self.from
    }

    pub fn to(&self) -> (usize, usize, PieceType) {
        self.to
    }

    pub fn to_f32(&self) -> (f32, f32) {
        (self.to.0 as f32, self.to.1 as f32)
    }

    pub fn inside(&self, x: usize, y: usize) -> bool {
        x == self.to.0 && y == self.to.1
    }
}
