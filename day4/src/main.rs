use std::collections::{HashMap, HashSet};
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    println!("Point sum for the cards is: {}", sum_points_from_cards(&input));
    println!("Count of all the cards is: {:?}", count_total_scratchcards(&input));
}

#[test]
fn count_cards_points() {
    let input = std::fs::read_to_string("data/test.txt").unwrap();
    assert_eq!(13, sum_points_from_cards(&input));

}

#[test]
fn count_total_cards(){
    let input = std::fs::read_to_string("data/test.txt").unwrap();
    assert_eq!(30, count_total_scratchcards(&input));
}

fn sum_points_from_cards(input: &str) -> u32{
    let mut sum: u32 = 0;
    let mut winning = HashSet::new();
    input.lines().for_each(|line|{
        let mut iter = line.split(':').last().unwrap().split('|');
        iter.next().unwrap().split_whitespace().for_each(|number| {
            winning.insert(number.parse::<u32>().unwrap());
        });
        let mut point_worth = 1;
        iter.next().unwrap().split_whitespace().for_each(|digit| {
            if winning.contains(&digit.parse::<u32>().unwrap()) {
                point_worth *= 2;
            }
        });
        sum += point_worth/2;
        winning.clear();
    }
    );
    sum
}


fn count_total_scratchcards(input: &str) -> anyhow::Result<u32> {
    let mut cache = HashMap::new();
    let mut sum = 0;
    let regex = regex::Regex::new(r"(?<winnings>( +\d*)*) +\| +(?<digits>(\d+ *)*)").unwrap();
    let lines:Vec<&str> = input.lines().collect();
    let len = lines.len();
    lines.iter().rev().enumerate().for_each(|(id,card)|{
        // Count the current card
        sum +=1;
        let found = regex.captures(card).unwrap();
        let winnings = parse_numbers::<HashSet::<u32>>(&found["winnings"]);
        let mut numbers = parse_numbers::<Vec<u32>>(&found["digits"]);
        numbers.retain(|number| winnings.contains(number));
        let mut win_score = numbers.len();

        for id_off in 0..numbers.len(){
            if let Some(number) = cache.get(&(len-id+id_off+1)) {
                win_score += number;
            }
        }
        cache.insert(len-id, win_score);
        sum += win_score;

    });
    u32::try_from(sum).map_err(|_| anyhow::anyhow!("Failed to convert"))
}

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

fn parse_numbers<T>(card_side: &str) -> T where T: FromIterator<u32> {
    NUMBER_REGEX.find_iter(card_side).filter_map(|mat| mat.as_str().parse::<u32>().ok()).collect()
}