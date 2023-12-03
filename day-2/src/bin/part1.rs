use std::{collections::HashMap, ops::AddAssign};

fn main() {
    let input = include_str!("input.txt");
    let res: i64 = process(input).iter().sum();
    println!("{}", res)
}
fn process(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|line| {
            let (game, rest) = line.split_once(":").unwrap();
            let game = game.split_whitespace().last().unwrap().parse().unwrap();

            let groups = rest.split(";");
            for group in groups {
                let mut group_nums: HashMap<&str, i64> = HashMap::new();
                for roll in group.split(",") {
                    let (num, color) = roll.trim().split_once(" ").unwrap();
                    group_nums
                        .entry(color)
                        .or_default()
                        .add_assign(num.parse::<i64>().unwrap())
                }
                if group_nums.get("blue").cloned().unwrap_or_default() > 14 {
                    return None;
                }
                if group_nums.get("red").cloned().unwrap_or_default() > 12 {
                    return None;
                }
                if group_nums.get("green").cloned().unwrap_or_default() > 13 {
                    return None;
                }
            }

            Some(game)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let res: i64 = process(input).iter().sum();
        assert_eq!(8, res)
    }
}
