/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!()
}

fn is_five_of_a_kind(hand_cards: &[&str]) -> bool {
    if hand_cards.len() != 5 {
        false
    } else {
        let first_card = hand_cards[0];
        let first_card_rank = get_card_rank(first_card);

        // e.g. card is 3S; with chars().next().unwrap() we get the first char, that is the rank
        for &card in hand_cards {
            if get_card_rank(card) != first_card_rank {
                return false;
            }
        }
        true
    }
}

fn is_straight_flush(hand_cards: &[&str]) -> bool {
    if hand_cards.len() != 5 {
        false
    } else {
        let ascending_scale_ranks = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let descending_scale_ranks = vec![14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        let first_card = hand_cards[0];
        let suit = get_card_suit(first_card);

        for card in hand_cards {
            if get_card_suit(card) != suit {
                // we found a card with a different suit
                return false;
            }
        }

        let first_card_rank = get_card_rank(first_card);

        let mut first_card_rank_in_scale = ascending_scale_ranks
            .iter()
            .position(|&rank| rank == first_card_rank)
            .unwrap();
        let scale_ranks = if first_card_rank_in_scale >= ascending_scale_ranks.len() - 5 {
            first_card_rank_in_scale = descending_scale_ranks
                .iter()
                .position(|&rank| rank == first_card_rank)
                .unwrap();

            descending_scale_ranks
        } else {
            ascending_scale_ranks
        };

        for &scale_rank in scale_ranks.iter().skip(first_card_rank_in_scale).take(5) {
            let mut rank_found = false;
            for card in hand_cards {
                if get_card_rank(card) == scale_rank {
                    // we found a card with the "scale rank"
                    rank_found = true;
                    break;
                }
            }

            if !rank_found {
                return false;
            }
        }

        true
    }
}

fn compute_hand_rank(hand_cards: &[&str], hand_type: PokerHandType) -> u8 {
    match hand_type {
        PokerHandType::FiveOfAKind => compute_five_of_a_kind_rank(hand_cards),
        PokerHandType::StraightFlush => compute_straight_flush_rank(hand_cards),
        _ => 0,
    }
}

fn compute_five_of_a_kind_rank(hand_cards: &[&str]) -> u8 {
    get_card_rank(hand_cards[0])
}

fn compute_straight_flush_rank(hand_cards: &[&str]) -> u8 {
    hand_cards
        .iter()
        .map(|card| get_card_rank(card))
        .fold(0, |prev, curr| if curr > prev { curr } else { prev })
}

pub fn get_card_suit(card: &str) -> &str {
    if card.len() > 1 {
        card.get(card.len() - 1..).unwrap()
    } else {
        card
    }
}

pub fn get_card_rank(card: &str) -> u8 {
    let rank_as_str = if card.len() > 1 {
        card.get(..card.len() - 1).unwrap()
    } else {
        card
    };

    match rank_as_str.parse::<u8>() {
        Ok(rank) => rank,
        Err(_) => match rank_as_str {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => panic!("unhandled card_rank = {}", rank_as_str),
        },
    }
}

pub fn decode_hand<'a>(hand: &'a str) -> PokerHand {
    let hand_cards: Vec<&str> = hand.split(' ').collect();

    let hand_type = if is_five_of_a_kind(&hand_cards) {
        PokerHandType::FiveOfAKind
    } else if is_straight_flush(&hand_cards) {
        PokerHandType::StraightFlush
    } else {
        PokerHandType::Unknown
    };

    let hand_rank = compute_hand_rank(&hand_cards, hand_type);

    PokerHand {
        hand,
        hand_type,
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
    pub rank: u8,
}
