use std::num;

fn main() {
    let input = include_str!("input.txt");
    let res: usize = process(input).iter().sum();
    println!("{}", res)
}
fn process(input: &str) -> Vec<usize> {
    let schematic: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            return line
                .chars()
                .map(|f| match f {
                    '.' => Slot::Empty,
                    '*' => Slot::Gear,
                    f if f.is_ascii_digit() => Slot::Didgit(f),
                    _ => Slot::Empty,
                })
                .collect();
        })
        .collect();

    let mut gears : Vec<usize> = vec![];
    for (y, row) in schematic.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if val == &Slot::Gear {
                let mut potential_ratios = vec![];
                let num = read_digit_at_offset(&schematic, x, y, 0, -1);
                potential_ratios.push(num);
                if num.is_none() {
                    potential_ratios.push(read_digit_at_offset(&schematic, x, y, -1, -1));
                    potential_ratios.push(read_digit_at_offset(&schematic, x, y, 1, -1));
                }
                potential_ratios.push(read_digit_at_offset(&schematic, x, y, -1, 0));
                potential_ratios.push(read_digit_at_offset(&schematic, x, y, 1, 0));

                let num = read_digit_at_offset(&schematic, x, y, 0, 1);
                potential_ratios.push(num);
                if num.is_none() {
                    potential_ratios.push(read_digit_at_offset(&schematic, x, y, -1, 1));
                    potential_ratios.push(read_digit_at_offset(&schematic, x, y, 1, 1));
                }

                if potential_ratios.iter().filter_map(|f| *f).count() == 2 {
                    gears.push(potential_ratios.iter().filter_map(|f| *f).product())
                }
            }
        
        }
    }
    gears
}

fn check_slot(
    vec: &Vec<Vec<Slot>>,
    x: usize,
    y: usize,
    offset_x: i8,
    offset_y: i8,
) -> Option<&Slot> {
    let y_to_check: usize = (y as i64 + offset_y as i64).try_into().ok()?;
    let x_to_check: usize = (x as i64 + offset_x as i64).try_into().ok()?;
    let y = vec.get(y_to_check)?;
    let x = y.get(x_to_check)?;
    return Some(x);
}
fn read_digit_at_offset(
    vec: &Vec<Vec<Slot>>,
    x: usize,
    y: usize,
    offset_x: i8,
    offset_y: i8,
) -> Option<usize> {
    let y_to_check: usize = (y as i64 + offset_y as i64).try_into().ok()?;
    let x_to_check: usize = (x as i64 + offset_x as i64).try_into().ok()?;
    return read_number(vec, x_to_check, y_to_check);
}
fn read_number(vec: &Vec<Vec<Slot>>, x: usize, y: usize) -> Option<usize> {
    let Some(&Slot::Didgit(did)) = check_slot(vec, x, y, 0, 0) else {
       return None; 
    };
    let mut str = did.to_string(); 

    let mut offset = -1;
    while let Some(&Slot::Didgit(did)) = check_slot(vec, x, y, offset, 0) {
        str = format!("{}{}", did, str);
        offset -= 1;
    }
    offset = 1;
    while let Some(&Slot::Didgit(did)) = check_slot(vec, x, y, offset, 0) {
        str = format!("{}{}", str, did);
        offset += 1;
    }

    str.parse().ok()
}
#[derive(PartialEq)]
enum Slot {
    Didgit(char),
    Gear,
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        println!("{:?}", process(input));
        let res: usize = process(input).iter().sum();
        assert_eq!(467835, res)
    }
}
