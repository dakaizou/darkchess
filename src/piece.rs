use crate::game::Color;
use crate::rank::Rank;

#[derive(Clone, PartialEq)]
pub struct Piece {
    rank: &'static Rank,
    color: Color,
    is_revealed: bool,
}

impl Piece {
    pub fn new(rank: &'static Rank, color: Color) -> Self {
        Self {
            rank,
            color,
            is_revealed: false,
        }
    }

    pub fn can_attack(&self, other: &Piece) -> bool {
        self.rank.can_attack(other.rank)
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    pub fn rank(&self) -> &Rank {
        self.rank
    }

    pub fn reveal(&mut self) {
        self.is_revealed = true;
    }
}
