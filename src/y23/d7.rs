use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

pub fn run(file_path: &str) {
    let hands = fs::read_to_string(file_path).expect("Failed to get input!");

    p1(&hands);
    p2(&hands);
}

fn p1(hands: &str) {
    let mut hands = hands.lines().map(|s| Hand::new(s, false)).collect::<Vec<Hand>>();
    
    hands.sort();
    let sum = hands.iter().enumerate().fold(0, |acc, (i, card)| {
        acc + (i as u64 + 1) * card.bid
    });
    println!("Sum: {sum}");

}

fn p2(hands: &str) {
    let mut hands = hands.lines().map(|s| Hand::new(s, true)).collect::<Vec<Hand>>();
    
    hands.sort();
    let sum = hands.iter().enumerate().fold(0, |acc, (i, card)| {
        acc + (i as u64 + 1) * card.bid
    });
    println!("Sum: {sum}");

}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}


struct Hand<'a> {
    cards: &'a str,
    hand_type: HandType,
    bid: u64,
    use_wildcards: bool,
}

impl<'a> Hand<'a> {
    fn new(line: &str, use_wildcards: bool) -> Hand {
        let (cards, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse::<u64>().unwrap();

        let hand_type;
        match use_wildcards {
            true => hand_type = get_max_type_from_cards(cards),
            false => hand_type = get_type_from_cards(cards),
        };

        Hand {
            cards,
            hand_type,
            bid,
            use_wildcards,
        }
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let strengths;
        match self.use_wildcards {
            true => strengths = vec!['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'],
            false => strengths = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'J', 'T', 'Q', 'K', 'A'],
        }

        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let mut self_strengths = Vec::new();
                for c in self.cards.chars() {
                    self_strengths.push(strengths.iter().position(|&ch| ch == c).unwrap());
                }

                let mut other_strengths = Vec::new();
                for c in other.cards.chars() {
                    other_strengths.push(strengths.iter().position(|&ch| ch == c).unwrap());
                }

                self_strengths.iter().cmp(other_strengths.iter())
            },
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        &self.cards == &other.cards
    }
}

impl<'a> Eq for Hand<'a> {
    
}

fn get_type_from_cards(cards: &str) -> HandType {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in cards.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let first = cards.chars().next().unwrap();

    if counts.len() == 1 {
        return HandType::FiveOfAKind;
    } else if counts.len() == 2 {
        if vec![1, 4].contains(&counts[&first]) {
            return HandType::FourOfAKind;
        } else {
            return HandType::FullHouse;
        }
    } else if counts.len() == 3 {
        for c in cards.chars() {
            if counts[&c] == 3 {
                return HandType::ThreeOfAKind;
            }
        }

        return HandType::TwoPair;
    } else if counts.len() == 4 {
        return HandType::OnePair;
    } else if counts.len() == 5 {
        return HandType::HighCard;
    }

    return HandType::HighCard;
}

fn get_max_type_from_cards(cards: &str) -> HandType {
    if !cards.contains('J') {
        return get_type_from_cards(cards);
    }

    let mut max_type = HandType::HighCard;
    let mut cards = cards.chars().collect::<Vec<char>>();
    
    max_type = find_max(&mut cards, 0, max_type);

    return max_type;
}

fn find_max(cards: &mut Vec<char>, index: usize, current_max: HandType) -> HandType {
    if index == cards.len() {
        return get_type_from_cards(cards.clone().into_iter().collect::<String>().as_str());
    } 

    if cards[index] != 'J' {
        return find_max(cards, index + 1, current_max);
    }

    static POSSIBLE_CARDS: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

    let mut max_type = current_max;
     
    for c in POSSIBLE_CARDS {
        cards[index] = c;
        max_type = std::cmp::max(max_type, find_max(cards, index + 1, max_type));
    }
    cards[index] = 'J';

    max_type
}
