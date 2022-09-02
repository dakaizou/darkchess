use crate::rank::Rank;


#[derive(PartialEq)]
pub struct Piece {
    rank: &'static Rank,
    is_black: bool,
    is_revealed: bool,
    is_selected: bool,
}

impl Clone for Piece {
    fn clone(&self) -> Self {
        Self {
            rank: self.rank,
            is_black: self.is_black,
            is_revealed: self.is_revealed,
            is_selected: self.is_selected,
        }
    }
}

impl Piece {
    pub fn new(rank: &'static Rank, is_black: bool) -> Self {
        Self {
            rank,
            is_black,
            is_revealed: false,
            is_selected: false,
        }
    }

    pub fn can_attack(&self, other: &Piece) -> bool {
        self.rank.can_attack(other.rank)
    }

    pub fn is_black(&self) -> bool {
        self.is_black
    }

    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    pub fn reveal(&mut self) {
        self.is_revealed = true;
    }
}