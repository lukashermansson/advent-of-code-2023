use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let res: i64 = process(input);
    println!("{}", res)
}
fn process<'a>(input: &'a str) -> i64 {
    let mut multipliers = HashMap::new();
    input
        .lines()
        .map(|line| {
            let (card_info, rest) = line.split_once(":").unwrap();
            let card_num: i64 = card_info
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let (board, winnings) = rest.split_once("|").unwrap();

            let (board, winnings) = (make_set(board), make_set(winnings));

            let points = board.iter().filter(|item| winnings.contains(item)).count() as i64;
            let this_mul = multipliers.get(&card_num).cloned().unwrap_or_default() + 1;
            (card_num + 1..=card_num + points).for_each(|card| {
                let value: &mut i64 = multipliers.entry(card).or_default();
                *value += this_mul
            });

            this_mul
        })
        .fold(0, |acc, e| acc + e)
}
fn make_set(input: &str) -> HashSet<i64> {
    input
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res: i64 = process(input);
        assert_eq!(30, res)
    }
}
