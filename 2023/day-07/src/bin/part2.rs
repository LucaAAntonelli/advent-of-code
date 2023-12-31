use std::{vec, collections::HashMap, cmp::Ordering};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum Card {
    J,
    Numeric(u32),
    T,
    Q,
    K,
    A
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            'A' => Self::A,
            'Q' => Self::Q,
            'J' => Self::J,
            'K' => Self::K,
            'T' => Self::T,
            other => Self::Numeric(other.to_digit(10).unwrap())
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,    
}

impl HandType {
    pub fn new(cards_str: &str) -> Self {
        let mut char_counts: HashMap<char, u8> = HashMap::new();

        for c in cards_str.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }
        if char_counts.contains_key(&'J') {
            // Handle Joker case
            let num_jokers = char_counts.get(&'J').unwrap();
            match num_jokers {
                5 => Self::FiveOfAKind, // Naturally
                4 => Self::FiveOfAKind, // Can always make 5 with fifth card
                3 => {
                    // Figure out if remaining two cards are equal
                    if char_counts.len() == 2 { // Two elements in map, one of them J with count 3 => Leftover pair
                        return Self::FiveOfAKind;
                    }
                    return Self::FourOfAKind; // Always possible with 3 Jokers
                }// 4, 5, full house
                2 => {
                    // If remaining three cards are equal => 5
                    if char_counts.len() == 2 { // Same as before, this time with J count 2, leftover three of a kind
                        return Self::FiveOfAKind;
                    }

                    // If remaining three cards are one pair + 1 => 4
                    if char_counts.len() == 3 {
                        return Self::FourOfAKind;
                    }

                    return Self::ThreeOfAKind; // Always possible with two cards
                }// 3, 4, 5, full house
                1 => {
                    // If remaining four cards are equal => 5
                    let values = char_counts.values().collect::<Vec<&u8>>();
                    if values.contains(&&4) {
                        return Self::FiveOfAKind;
                    } 

                    // If remaining four cards are three of a kind + 1 => 4
                    if values.contains(&&3) {
                        return Self::FourOfAKind;
                    }

                    // If remaining four cards are two pairs => Full House
                    if char_counts.len() == 3 {
                        return Self::FullHouse;
                    }

                    // If remaining four cards are one pair + 2 => 3
                    if values.contains(&&2) {
                        return Self::ThreeOfAKind;
                    }

                    return Self::OnePair;
                }// 2, 3, 4, 5, full house
                _ => {panic!("Shouldn't be possible");}
            };

            Self::FiveOfAKind
        } else {
            let values: Vec<&u8> = char_counts.values().collect();
        if values.contains(&&5) {
            Self::FiveOfAKind
        } else if values.contains(&&4) {
            Self::FourOfAKind
        } else if values.contains(&&3) && values.contains(&&2) {
            Self::FullHouse
        } else if values.contains(&&3) {
            Self::ThreeOfAKind
        } else if values.iter().filter(|&x| x == &&2).count() == 2 { // Check if 2 is contained twice
            Self::TwoPairs
        } else if values.contains(&&2) {
            Self::OnePair
        } else {
            Self::HighCard
        }
        }
        
    }
}



#[derive(Debug, Eq, PartialEq)]
struct Hand {
    card_1: Card,
    card_2: Card,
    card_3: Card,
    card_4: Card,
    card_5: Card,
    bid: u32,
    hand_type: HandType
}

impl Hand {
    pub fn new(line: &str) -> Self {
        println!("{line}");
        let mut split = line.split_whitespace();
        let cards = split.next().unwrap();
        let bid = split.next().unwrap().parse::<u32>().unwrap();
        let mut cards_as_chars = cards.chars();
        let card_1 = Card::new(cards_as_chars.next().unwrap());
        let card_2 = Card::new(cards_as_chars.next().unwrap());
        let card_3 = Card::new(cards_as_chars.next().unwrap());
        let card_4 = Card::new(cards_as_chars.next().unwrap());
        let card_5 = Card::new(cards_as_chars.next().unwrap());
        let hand_type = HandType::new(cards);
        Self {
            card_1,
            card_2,
            card_3,
            card_4,
            card_5,
            bid,
            hand_type
        }
    }

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            Some(Ordering::Greater)
        } else if self.hand_type < other.hand_type {
            Some(Ordering::Less)
        } else {
            // Do logic to find larger of first card
            if self.card_1 > other.card_1 {
                Some(Ordering::Greater)
            } else if self.card_1 < other.card_1 {
                Some(Ordering::Less)
            } else {
                if self.card_2 > other.card_2 {
                    Some(Ordering::Greater) 
                } else if self.card_2 < other.card_2 {
                    Some(Ordering::Less)
                } else {
                    if self.card_3 > other.card_3 {
                        Some(Ordering::Greater)
                    } else if self.card_3 < other.card_3 {
                        Some(Ordering::Less)
                    } else {
                        if self.card_4 > other.card_4 {
                            Some(Ordering::Greater)
                        } else if self.card_4 < other.card_4 {
                            Some(Ordering::Less)
                        } else {
                            if self.card_5 > other.card_5 {
                                Some(Ordering::Greater)
                            } else if self.card_5 < other.card_5 {
                                Some(Ordering::Less)
                            } else {
                                panic!("All five cards are equal");
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.hand_type < other.hand_type {
            Ordering::Less
        } else {
            // Do logic to find larger of first card
            if self.card_1 > other.card_1 {
                Ordering::Greater
            } else if self.card_1 < other.card_1 {
                Ordering::Less
            } else {
                if self.card_2 > other.card_2 {
                    Ordering::Greater 
                } else if self.card_2 < other.card_2 {
                    Ordering::Less
                } else {
                    if self.card_3 > other.card_3 {
                        Ordering::Greater
                    } else if self.card_3 < other.card_3 {
                        Ordering::Less
                    } else {
                        if self.card_4 > other.card_4 {
                            Ordering::Greater
                        } else if self.card_4 < other.card_4 {
                            Ordering::Less
                        } else {
                            if self.card_5 > other.card_5 {
                                Ordering::Greater
                            } else if self.card_5 < other.card_5 {
                                Ordering::Less
                            } else {
                                panic!("All five cards are equal");
                            }
                        }
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let lines = input.lines();
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        let hand = Hand::new(line);
        hands.push(hand);
    }
    let mut result: usize = 0;
    hands.sort();
    for (index, hand) in hands.iter().enumerate() {
        result += hand.bid as usize * (index+1);
    }
    result
    
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        );
        assert_eq!(result, 5905);
    }

}
