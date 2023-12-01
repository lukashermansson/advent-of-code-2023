
use regex::{Regex};

fn main() {
    let input = include_str!("input.txt").to_string();
    let res: i64 = process(input).iter().sum();
    println!("{}", res)
}

fn process(input: String) -> Vec<i64> {
    let lines = input.lines();
    lines.map(|line| {
        let regexes = vec![
            ("0", '0'),
            ("1", '1'),
            ("2", '2'),
            ("3", '3'),
            ("4", '4'),
            ("5", '5'),
            ("6", '6'),
            ("7", '7'),
            ("8", '8'),
            ("9", '9'),
            ("zero", '0'),
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ];
        let mut shortest: (Option<usize>, Option<char>) = (None, None);
        let mut longest: (Option<usize>, Option<char>) = (None, None);
        for regex in regexes {
            let mat: Vec<_> = Regex::new(regex.0).unwrap().find_iter(&line).collect();

            match mat.iter().nth(0) {
                Some(shortest_match) => match shortest.0 {
                    Some(srt) => {
                        if srt > shortest_match.start() {
                            shortest = (Some(shortest_match.start()), Some(regex.1))
                        }
                    }
                    None => shortest = (Some(shortest_match.start()), Some(regex.1)),
                },
                None => {}
            }
            match mat.iter().last() {
                Some(longest_match) => match longest.0 {
                    Some(long) => {
                        if long <= longest_match.start() {
                            longest = (Some(longest_match.start()), Some(regex.1))
                        }
                    }
                    None => longest = (Some(longest_match.start()), Some(regex.1)),
                },
                None => {}
            }
        }
        let out = format!("{}{}", shortest.1.unwrap(),  &longest.1.unwrap());

        out.parse().unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();
        let res: i64 = process(input).iter().sum();
        assert_eq!(281, res)
    }
}
