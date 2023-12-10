use std::{collections::HashSet, error::Error};

// Module for day 4

struct Card {
    matches: u32,
    copies: u32,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt").lines();

    let mut cards: Vec<Card> = vec![];

    let data = input.map(|card| {
        card.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|nums| {
                nums.split(" ")
                    .filter(|x| *x != "")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>()
    });

    let mut winning_pts_sum = 0;

    for card in data {
        let matches: u32 = get_matches_count(card);

        let base: u32 = 2;

        if matches > 0 {
            winning_pts_sum += base.pow(matches - 1);
        }

        cards.push(Card { matches, copies: 1 });
    }

    let mut copies_sum = 0;

    let card_count = cards.len();

    for i in 0..card_count {
        let curr_card_matches = cards.get(i).unwrap().matches;
        let curr_card_copies = cards.get(i).unwrap().copies;

        let end = if i + 1 + curr_card_matches as usize > card_count {
            card_count
        } else {
            i + 1 + curr_card_matches as usize
        };

        for j in (i + 1)..end {
            cards[j].copies += curr_card_copies; // one pt for each copy of the card
        }

        copies_sum += curr_card_copies;
    }

    println!("Winning points sum: {}", winning_pts_sum);

    println!("Total card copies: {}", copies_sum);

    Ok(())
}

fn get_matches_count(curr_card: Vec<Vec<u32>>) -> u32 {
    let (winning_numbers, my_numbers) = (curr_card.get(0).unwrap(), curr_card.get(1).unwrap());

    let winning_numbers: HashSet<_> = winning_numbers.iter().collect();

    let my_numbers: HashSet<_> = my_numbers.iter().collect();

    return winning_numbers.intersection(&my_numbers).count() as u32;
}
