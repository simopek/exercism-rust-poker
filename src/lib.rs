/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!()
}

fn is_five_of_a_kind(hand_cards: &Vec<&str>) -> bool {

    if hand_cards.len() != 5 {
        false
    } else {
        let first_card = hand_cards[0];

        // e.g. card is 3S; with chars().next().unwrap() we get the first char, that is the rank
        for &card in hand_cards {
            if card.chars().next().unwrap() != first_card.chars().next().unwrap() {
                return false;
            }
        }
        true
    }
}

fn compute_hand_rank(hand_cards: &Vec<&str>, hand_type: PokerHandType) -> u32 {
    match hand_type {
        PokerHandType::FiveOfAKind => compute_five_of_a_kind_rank(hand_cards),
        _ => 0,
    }
}

fn compute_five_of_a_kind_rank(hand_cards: &Vec<&str>) -> u32 {
    let card_rank = hand_cards[0].chars().next().unwrap();

    if card_rank.is_numeric() {
        card_rank.to_digit(10).unwrap()
    } else {
        match card_rank {
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("unhandled card_rank = {}", card_rank),
        }
    }
}

pub fn decode_hand<'a>(hand: &'a str) -> PokerHand {
    let hand_cards: Vec<&str> = hand.split(" ").collect();

    let hand_type = if is_five_of_a_kind(&hand_cards) {
        PokerHandType::FiveOfAKind
    } else {
        PokerHandType::Unknown
    };

    let hand_rank = compute_hand_rank(&hand_cards, hand_type);

    PokerHand {
        hand,
        hand_type: hand_type,
        rank: hand_rank,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PokerHandType {
    FiveOfAKind,
    StraightFlush,
    Unknown,
}

pub struct PokerHand<'a> {
    hand: &'a str,
    pub hand_type: PokerHandType,
    pub rank: u32,
}
