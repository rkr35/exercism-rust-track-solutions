const NUM_SQUARES_PER_SIDE: i32 = 8;

#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let f = |x: i32| if (0..NUM_SQUARES_PER_SIDE).contains(&x) { Some(x) } else { None };
        Some(Self(f(rank)?, f(file)?))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let ChessPosition(r0, c0) = self.0;
        let ChessPosition(r1, c1) = other.0;
        r0 == r1 || c0 == c1 || (r1 - r0).abs() == (c1 - c0).abs()
    }
}
