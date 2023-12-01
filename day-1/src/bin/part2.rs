fn main() {
    let input = include_str!("input.txt").to_string();
    let res: i64 = process(input).iter().sum();
    println!("{}", res)
}

fn process(input: String) -> Vec<i64> {
    let regexes = vec![
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
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
    let lines = input.lines();
    lines
        .map(|line| {
            let rev_line: String = line.chars().rev().collect();
            let rev_matches: Vec<(String, char)> = regexes
                .iter()
                .map(|f| (f.0.chars().rev().collect(), f.1))
                .collect();

            let out = format!(
                "{}{}",
                find_first(&regexes, line),
                find_first(&rev_matches, &rev_line),
            );
            out.parse().unwrap()
        })
        .collect()
}
fn find_first<S: AsRef<str>>(options: &Vec<(S, char)>, haystack: &str) -> char {
    let mut first_idx = 9999;
    let mut char = None;
    options
        .iter()
        .for_each(|f| {
            if let Some(min) = haystack.find(f.0.as_ref()) {

            if min < first_idx {
                first_idx = min;
                char = Some(f.1)
            }
            }
        });

       char.unwrap()
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

    #[test]
    fn more_input() {
        let input = "oneight".to_string();
        let res: i64 = process(input).iter().sum();
        assert_eq!(18, res)
    }
}
