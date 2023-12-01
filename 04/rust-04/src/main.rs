#[derive(Clone, Debug)]
pub struct Card {
    id: u16,
    winning_numbers: Vec<u8>,
    player_numbers: Vec<u8>,
}

fn parse_line(line: &str) -> Card {
    let (card, rest) = line.split_once(':').unwrap();
    let (_, card_id) = card.split_once(' ').unwrap();
    let card_id = card_id.trim().parse::<u16>().unwrap();
    let (winning_numbers, player_numbers) = rest.split_once('|').unwrap();

    let winning_numbers = winning_numbers
        .trim()
        .split(' ')
        .filter(|str| !str.is_empty())
        .map(|number| number.trim().parse::<u8>().unwrap())
        .collect();

    let player_numbers = player_numbers
        .trim()
        .split(' ')
        .filter(|str| !str.is_empty())
        .map(|number| number.trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    Card {
        id: card_id,
        winning_numbers,
        player_numbers,
    }
}

mod part1 {
    use crate::Card;

    pub fn get_winning_numbers(card: &Card) -> Vec<u8> {
        card.player_numbers
            .clone()
            .into_iter()
            .filter(|number| card.winning_numbers.contains(number))
            .collect::<Vec<u8>>()
    }

    pub fn solve(cards: &[Card]) -> u32 {
        cards
            .iter()
            .map(get_winning_numbers)
            .map(|numbers| {
                if !numbers.is_empty() {
                    2u32.pow(numbers.len() as u32 - 1)
                } else {
                    0
                }
            })
            .sum()
    }
}

mod part2 {
    use crate::{part1, Card};
    use std::collections::BTreeMap;

    type CardState = BTreeMap<u16, CardInstance>;

    #[derive(Clone, Debug)]
    struct CardInstance {
        total_instances: u64,
        card: Card,
    }

    fn build_cards_hashmap(cards: &[Card]) -> CardState {
        cards
            .iter()
            .map(|card| {
                (
                    card.id,
                    CardInstance {
                        total_instances: 1,
                        card: card.clone(),
                    },
                )
            })
            .collect()
    }

    pub fn solve(cards: &[Card]) -> u64 {
        let mut state = build_cards_hashmap(cards);
        let mut card_id = 1;

        loop {
            let instance = state.get(&card_id).unwrap();
            let won_tickets = instance.total_instances;

            if state.len() == card_id as usize {
                break;
            }

            let winning_numbers = part1::get_winning_numbers(&instance.card).len();

            state
                .iter_mut()
                .skip(card_id as usize)
                .take(winning_numbers)
                .for_each(|(_, instance)| {
                    instance.total_instances += won_tickets;
                });

            card_id += 1;
        }

        state
            .values()
            .map(|instance| instance.total_instances)
            .sum()
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("usage: {} <filename>", args[0]);
        return;
    }

    let contents = std::fs::read_to_string(&args[1])
        .unwrap_or_else(|_| panic!("could not read file {}", args[1]));

    let cards = contents.lines().map(parse_line).collect::<Vec<_>>();

    let result_1 = part1::solve(&cards);
    let result_2 = part2::solve(&cards);

    println!("part 1: {}", result_1);
    println!("part 2: {}", result_2);
}
