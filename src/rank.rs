#[derive(PartialEq, PartialOrd, Debug)]
pub enum Rank {
    General,
    Advisor,
    Elephant,
    Chariot,
    Horse,
    Cannon,
    Sodier,
}

impl Rank {
    pub fn can_attack(&self, other: &Rank) -> bool {
        match *self {
            Rank::Cannon => true,
            Rank::General => *other != Rank::Sodier,
            Rank::Sodier => *other == Rank::Sodier || *other == Rank::General,
            _ => self <= other,
        }
    }
}

pub const RANK_SET: [Rank; 16] = [
    Rank::General,
    Rank::Advisor,
    Rank::Advisor,
    Rank::Elephant,
    Rank::Elephant,
    Rank::Chariot,
    Rank::Chariot,
    Rank::Horse,
    Rank::Horse,
    Rank::Cannon,
    Rank::Cannon,
    Rank::Sodier,
    Rank::Sodier,
    Rank::Sodier,
    Rank::Sodier,
    Rank::Sodier,
];
