use std::collections::HashMap;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Hand {
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
enum PlayingCard {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Number(u8),
}

impl PlayingCard {

    fn compare_power(&self, other: &Self) -> std::cmp::Ordering {
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
        }
    }
    fn parse_vec_from_str(raw: &str) -> Vec<Self> {
        raw.chars()
            .map(|c | match c {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => Self::Jack,
                'T' => Self::Ten,
                _ => Self::Number(c.to_digit(10).unwrap_or(0) as u8),
            })
            .collect()

    }


    fn get_hand(cards: Vec<Self>) -> Hand {
    //check for 4 of a kind
    
    let mut card_count: HashMap<Self, u8> = HashMap::new();

    for card in cards {
        let count = card_count.entry(card).or_insert(0);
        *count += 1;
    }
    
    let mut card_count_tup: Vec<(Self, u8)> = card_count.into_iter().collect();
    card_count_tup.sort_by(|a, b| b.1.cmp(&a.1));

    if let Some((card, count)) = card_count_tup.first() {
        if *count == 5 {
            return Hand::FiveOfAKind(card.clone());
        }
        if  *count == 4 {
            return Hand::FourOfAKind(card.clone());
        }
        if *count == 3 {
            if let Some((card2, count2)) = card_count_tup.get(1) {
                if *count2 == 2 {
                    return Hand::FullHouse(card.clone(), card2.clone());
                }
            }
            return Hand::ThreeOfAKind(card.clone());
        }
        if *count == 2 {
            if let Some((card2, count2)) = card_count_tup.get(1) {
                if *count2 == 2 {
                    return Hand::TwoPair(card.clone(), card2.clone());
                }
            }
            return Hand::Pair(card.clone());
        }
        return Hand::HighCard(card.clone());
    }
    return Hand::HighCard(PlayingCard::Number(0));
    
}



}