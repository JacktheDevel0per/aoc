use std::collections::HashMap;
use itertools::Itertools;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hand {
    FiveOfAKind(PlayingCard),
    FourOfAKind(PlayingCard),
    FullHouse(PlayingCard, PlayingCard),
    ThreeOfAKind(PlayingCard),
    TwoPair(PlayingCard, PlayingCard),
    Pair(PlayingCard),
    HighCard(PlayingCard),
}



impl Hand {
    fn better_hand(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match (self, other) {
            (Self::FiveOfAKind(a), Self::FiveOfAKind(b)) => a.compare_power(b),
            (Self::FiveOfAKind(_), _) => Greater,
            (_, Self::FiveOfAKind(_)) => Less,
            (Self::FourOfAKind(a), Self::FourOfAKind(b)) => a.compare_power(b),
            (Self::FourOfAKind(_), _) => Greater,
            (_, Self::FourOfAKind(_)) => Less,
            (Self::FullHouse(a, b), Self::FullHouse(c, d)) => {
                match a.compare_power(c) {
                    Equal => b.compare_power(d),
                    x => x,
                }
            }
            (Self::FullHouse(_,_), _) => Greater,
            (_, Self::FullHouse(_, _)) => Less,
            (Self::ThreeOfAKind(a), Self::ThreeOfAKind(b)) => a.compare_power(b),
            (Self::ThreeOfAKind(_), _) => Greater,
            (_, Self::ThreeOfAKind(_)) => Less,
            (Self::TwoPair(a, b), Self::TwoPair(c, d)) => {
                match a.compare_power(c) {
                    Equal => b.compare_power(d),
                    x => x,
                }
            }
            (Self::TwoPair(_,_), _) => Greater,
            (_, Self::TwoPair(_, _)) => Less,
            (Self::Pair(a), Self::Pair(b)) => a.compare_power(b),
            (Self::Pair(_), _) => Greater,
            (_, Self::Pair(_)) => Less,
            (Self::HighCard(a), Self::HighCard(b)) => a.compare_power(b),
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayingCard {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Number(u8),
    Joker
}

impl PlayingCard {

    pub fn compare_power(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match (self, other) {
            (Self::Ace, _) => Greater,
            (_, Self::Ace) => Less,
            (Self::King, _) => Greater,
            (_, Self::King) => Less,
            (Self::Queen, _) => Greater,
            (_, Self::Queen) => Less,
            (Self::Jack, _) => Greater,
            (_, Self::Jack) => Less,
            (Self::Ten, _) => Greater,
            (_, Self::Ten) => Less,
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::Joker, Self::Joker) => Equal,
            (Self::Joker, _) => Greater,
            (_, Self::Joker) => Less,
        }
    }
    pub fn parse_vec_from_str(raw: &str) -> Vec<Self> {
        raw.chars()
            .map(|c | match c {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => Self::Jack, //in part 2, this is a joker
                'T' => Self::Ten,

                _ => Self::Number(c.to_digit(10).unwrap_or(0) as u8),
            })
            .collect()

    }


    pub fn get_hand(cards: [Self; 5]) -> (Hand, [Self; 5]) {

    let mut card_count: HashMap<Self, u8> = HashMap::new();

    for card in cards {
        let count = card_count.entry(card).or_insert(0);
        *count += 1;
    }
    
    let mut card_count_tup: Vec<(Self, u8)> = card_count.into_iter().collect();
    card_count_tup.sort_by(|a, b| b.1.cmp(&a.1));

    let joker_count = card_count_tup.iter().filter(|(card, _)| *card == Self::Joker).count() as u8;

    if let Some((card, count)) = card_count_tup.iter().find(| card| card.0 != Self::Joker) {
        if *count + joker_count == 5 {
            return (Hand::FiveOfAKind(card.clone()), cards);
        }
        if  *count + joker_count == 4 {
            return (Hand::FourOfAKind(card.clone()), cards);
        }
        if *count + joker_count == 3 {
            if let Some((card2, count2)) = card_count_tup.iter().find( | other_card_and_count | other_card_and_count.0 != Self::Joker && *card != other_card_and_count.0) {
                if *count2 == 2 {
                    return (Hand::FullHouse(card.clone(), card2.clone()), cards);
                }
            }
            return (Hand::ThreeOfAKind(card.clone()), cards);
        }
        if *count + joker_count == 2 {
            if let Some((card2, count2)) = card_count_tup.iter().find( | other_card_and_count | other_card_and_count.0 != Self::Joker && *card != other_card_and_count.0) {
                if *count2 == 2 {
                    return (Hand::TwoPair(card.clone(), card2.clone()), cards);
                }
            }
            return (Hand::Pair(card.clone()), cards);
        }
        return (Hand::HighCard(card.clone()), cards);
    }
    //Should not come to this, I don't want to undwap the find though.
    return (Hand::HighCard(PlayingCard::Number(0)), cards);
    
}


fn sort_by_strength(hands: Vec<(Hand, [PlayingCard; 5])>) -> Vec<(Hand, [PlayingCard; 5])> {
    hands.iter().sorted_by(|a, b| {
    let ordering = a.0.better_hand(&b.0);

    if ordering == std::cmp::Ordering::Equal {
        for i in 0..5 {
            let new_ordering = a.1[i].compare_power(&b.1[i]);
            if new_ordering != std::cmp::Ordering::Equal {
                return new_ordering;
            }
        }
    }
    return ordering;

    }).cloned().collect()

}



}