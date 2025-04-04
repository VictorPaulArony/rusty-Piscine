use rand::Rng;

#[derive(PartialEq, Debug,Copy, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng =  rand::thread_rng();
        let suit = [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club];
        let random_index =rng.gen_range(0..suit.len());
        suit[random_index]
    }

    pub fn translate(value: u8) -> Suit {
        match value{
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Incalid")
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng =  rand::thread_rng();
        let rank = [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack];
        let random_index = rng.gen_range(0..rank.len());
        rank[random_index]
    }   

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2 => Rank::King,
            3 => Rank::Queen,
            4 => Rank::Jack,
            _ => panic!("Incalid")
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Suit::Club;
    use crate::Rank::Ace;

    #[test]
    fn it_works() {
        let your_card = Card {
            rank: Rank::random(),
            suit: Suit::random(),
        };
        assert_eq!(your_card, Card { suit: Club, rank: Ace });
    }
}
