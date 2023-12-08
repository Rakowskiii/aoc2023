use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::parse(line)).collect();
    hands.sort();
    println!("Total winnings: {}", total_winnings(&hands));
}

#[cfg(not(feature = "jokers"))]
const EXPECTED_TOTAL_WINNINGS: u64 = 6440;
#[cfg(feature = "jokers")]
const EXPECTED_TOTAL_WINNINGS: u64 = 5905;

#[test]
fn test_total_winnings() {
    let input = std::fs::read_to_string("data/test.txt").unwrap();
    let mut vec: Vec<_> = input.lines().map(|line| Hand::parse(line)).collect();
    vec.sort();
    assert_eq!(EXPECTED_TOTAL_WINNINGS, total_winnings(&vec));
}

fn total_winnings(hands: &[Hand]) -> u64 {
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| ((rank + 1) as u64) * hand.bet)
        .sum()
}

impl Hand {
    fn analyze_hand(cards: &[Card]) -> CardFigure {
        let mut counts = HashMap::new();

        #[cfg(feature = "jokers")]
            let mut jokers = 0;
        #[cfg(feature = "jokers")]
        {
            for &card in cards {
                if card == Card::Joker {
                    jokers += 1;
                }
            }
            if jokers == 5 {
                return CardFigure::FiveOfKind(Card::Ace);
            }
        }


        #[cfg(not(feature = "jokers"))]
        for &card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }
        #[cfg(feature = "jokers")]
        for &card in cards {
            if card != Card::Joker {
                *counts.entry(card).or_insert(0) += 1;
            }
        }

        // Sort counts by frequency and then by card value
        let mut count_vec: Vec<_> = counts.into_iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
        #[cfg(feature = "jokers")]
        {
            for i in 0..count_vec.len() {
                let card = count_vec.get_mut(i).unwrap();
                if card.0 != Card::Joker {
                    println!("adding {} jokers to {} of {}", jokers, card.1, card.0);
                    card.1 += jokers;
                    if card.1 > 5 {
                        card.1 = 5;
                    }
                    break;
                }
            }
        }

        match count_vec.as_slice() {
            [(card, 5), ..] => CardFigure::FiveOfKind(*card),
            [(card, 4), ..] => CardFigure::FourOfKind(*card),
            [(card1, 3), (card2, 2), ..] | [(card2, 2), (card1, 3), ..] => {
                CardFigure::FullHouse(*card1, *card2)
            }
            [(card, 3), ..] => CardFigure::ThreeOfKind(*card),
            [(card1, 2), (card2, 2), ..] => CardFigure::TwoPair(*card1, *card2),
            [(card, 2), ..] => CardFigure::Pair(*card),
            [(card, _), ..] => CardFigure::HighCard(*card),
            _ => panic!("Invalid hand"),
        }
    }
    fn parse_cards(cards: &str) -> [Card; 5] {
        let vec: Vec<_> = cards
            .chars()
            .map(|c| Card::from_char(&c).unwrap())
            .collect();
        vec.try_into().unwrap()
    }

    fn parse(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let hand = parts.next().unwrap();
        let bet = parts.next().unwrap().parse::<u64>().unwrap();
        let cards = Self::parse_cards(hand);
        let figure = Self::analyze_hand(&cards);

        Self { cards, figure, bet }
    }
}

trait FromFile: Sized {
    type Error;
    fn from_file(path: &str) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Card {
    #[cfg(feature = "jokers")]
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Token,
    #[cfg(not(feature = "jokers"))]
    Jack,
    Queen,
    King,
    Ace,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let card_char = match self {
            #[cfg(feature = "jokers")]
            Card::Joker => "J",
            Card::Two => "2",
            Card::Three => "3",
            Card::Four => "4",
            Card::Five => "5",
            Card::Six => "6",
            Card::Seven => "7",
            Card::Eight => "8",
            Card::Nine => "9",
            Card::Token => "T",
            #[cfg(not(feature = "jokers"))]
            Card::Jack => "J",
            Card::Queen => "Q",
            Card::King => "K",
            Card::Ace => "A",
        };

        write!(f, "{}", card_char)
    }
}

trait FromChar: Sized {
    type Error;
    fn from_char(c: &char) -> Result<Self, Self::Error>;
}

impl FromChar for Card {
    type Error = ();
    fn from_char(c: &char) -> Result<Self, ()> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            #[cfg(not(feature = "jokers"))]
            'J' => Ok(Self::Jack),
            #[cfg(feature = "jokers")]
            'J' => Ok(Self::Joker),
            'T' => Ok(Self::Token),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum CardFigure {
    FiveOfKind(Card),
    FourOfKind(Card),
    FullHouse(Card, Card),
    ThreeOfKind(Card),
    TwoPair(Card, Card),
    Pair(Card),
    HighCard(Card),
}

impl CardFigure {
    fn value(&self) -> u8 {
        match self {
            Self::FiveOfKind(_) => 7,
            Self::FourOfKind(_) => 6,
            Self::FullHouse(_, _) => 5,
            Self::ThreeOfKind(_) => 4,
            Self::TwoPair(_, _) => 3,
            Self::Pair(_) => 2,
            Self::HighCard(_) => 1,
        }
    }
}

impl Ord for CardFigure {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}


impl PartialOrd for CardFigure {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    figure: CardFigure,
    bet: Bet,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.figure.cmp(&other.figure) {
            ord @ std::cmp::Ordering::Less | ord @ std::cmp::Ordering::Greater => ord,
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    let ord = self.cards[i].cmp(&other.cards[i]);
                    if ord != std::cmp::Ordering::Equal {
                        return ord;
                    }
                }
                return std::cmp::Ordering::Equal;
            }
        }
    }
}

type Bet = u64;

