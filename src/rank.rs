#[derive(PartialEq, PartialOrd, Debug)]
pub enum Rank {
    King,
    General,
    Colonel,
    Major,
    Captain,
    Cannon,
    Sodier,
}

impl Rank {
    pub fn can_attack(&self, other: &Rank) -> bool {
        if *self == Rank::Cannon {
            return true;
        }

        if *self == Rank::King {
            return *other != Rank::Sodier;
        }

        if *self == Rank::Sodier {
            return *other == Rank::Sodier || *other == Rank::King;
        }

        self <= other
    }
}

pub const RANK_SET: [Rank; 16] = [
    Rank::King,
    Rank::General,
    Rank::General,
    Rank::Colonel,
    Rank::Colonel,
    Rank::Major,
    Rank::Major,
    Rank::Captain,
    Rank::Captain,
    Rank::Cannon,
    Rank::Cannon,
    Rank::Sodier,
    Rank::Sodier,
    Rank::Sodier,
    Rank::Sodier,
    Rank::Sodier,
];