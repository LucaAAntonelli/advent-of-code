use std::{vec, collections::HashMap, cmp::Ordering};

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2023/day-07.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum Card {
    Numeric(u32),
    T,
    J,
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
    ThreeOFAKind,
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
        let values: Vec<&u8> = char_counts.values().collect();
        if values.contains(&&5) {
            Self::FiveOfAKind
        } else if values.contains(&&4) {
            Self::FourOfAKind
        } else if values.contains(&&3) && values.contains(&&2) {
            Self::FullHouse
        } else if values.contains(&&3) {
            Self::ThreeOFAKind
        } else if values.iter().filter(|&x| x == &&2).count() == 2 { // Check if 2 is contained twice
            Self::TwoPairs
        } else if values.contains(&&2) {
            Self::OnePair
        } else {
            Self::HighCard
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
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_sorting() {
        let hand_1 = Hand::new("11111 56");
        let hand_2 = Hand::new("KKKK3 1");
        assert!(hand_1 > hand_2);
    }

    #[test]
    fn test_hand_ordering() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FiveOfAKind > HandType::ThreeOFAKind);
        assert!(HandType::FiveOfAKind > HandType::FullHouse);
        assert!(HandType::FiveOfAKind > HandType::TwoPairs);
        assert!(HandType::FiveOfAKind > HandType::OnePair);
        assert!(HandType::FiveOfAKind > HandType::HighCard);

        assert!(HandType::FourOfAKind > HandType::ThreeOFAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FourOfAKind > HandType::TwoPairs);
        assert!(HandType::FourOfAKind > HandType::OnePair);
        assert!(HandType::FourOfAKind > HandType::HighCard);

        assert!(HandType::FullHouse > HandType::ThreeOFAKind);
        assert!(HandType::FullHouse > HandType::TwoPairs);
        assert!(HandType::FullHouse > HandType::OnePair);
        assert!(HandType::FullHouse > HandType::HighCard);

        assert!(HandType::ThreeOFAKind > HandType::TwoPairs);
        assert!(HandType::ThreeOFAKind > HandType::OnePair);
        assert!(HandType::ThreeOFAKind > HandType::HighCard);

        assert!(HandType::TwoPairs > HandType::OnePair);
        assert!(HandType::TwoPairs > HandType::HighCard);

        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn test_cards() {
        assert!(Card::Numeric(3) < Card::Numeric(5));
        assert!(Card::A > Card::J);
        assert!(Card::Numeric(5) < Card::T);
    }
}
