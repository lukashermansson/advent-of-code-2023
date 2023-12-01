fn main() {
    let input = include_str!("input.txt");
    let res: i64 = process(input).iter().sum();
    println!("{}", res)
}
fn process<'a>(input: &'a str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            let str: Vec<char> = line
                .chars()
                .filter_map(|char| match char {
                    '0'..='9' => Some(char),
                    _ => None,
                })
                .collect();

            let first = str.first().unwrap();
            let last = str.last().unwrap();
            let out = format!("{}{}", first, last);
            out.parse().unwrap()
        })
        .collect::<Vec<i64>>()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let res: i64 = process(input).iter().sum();
        assert_eq!(142, res)
    }
}
