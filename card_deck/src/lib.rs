use rand::Rng;

#[derive(Debug,PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug,PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            _ => Suit::Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=13) {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Queen,
            _ => Rank::Jack,
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2 => Rank::King,
            3 => Rank::Queen,
            _ => Rank::Jack,
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    let _rng = rand::thread_rng();
    let winner = Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    };
    card == &winner || (card.suit == Suit::Spade && card.rank == Rank::Ace)
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//        #[test]
//     fn test_winner() {
//         let winner = Card {
//             rank: Rank::Ace,
//             suit: Suit::Spade,
//         };

//         for rank in 1..14 {
//             for suit in 1..5 {
//                 let card = Card {
//                     rank: Rank::translate(rank),
//                     suit: Suit::translate(suit),
//                 };

//                // assert_eq!(card_deck::winner_card(&card), card == winner);
//             }
//         }
//     }
// }
