advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let (lower_str, upper_str) = input.trim().split_once('-').unwrap();
    let (lower, upper) = (lower_str.parse::<u32>().unwrap(), upper_str.parse::<u32>().unwrap());

    let mut count: u32 = 0;
    for num in lower..upper {
        if is_valid(num) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (lower_str, upper_str) = input.trim().split_once('-').unwrap();
    let (lower, upper) = (lower_str.parse::<u32>().unwrap(), upper_str.parse::<u32>().unwrap());

    let mut count: u32 = 0;
    for num in lower..upper {
        if is_valid_part2(num) {
            count += 1;
        }
    }

    Some(count)
}

fn is_valid(num: u32) -> bool {
    let chars = num.to_string().chars().collect::<Vec<char>>();
    // two adjacent digits are the same
    let mut adjacent_nums = false;
    for window in chars.to_vec().windows(2) {
        if window[0] == window[1] {
            adjacent_nums = true;
            break;
        }
    }

    // decreasing
    let mut decreasing = false;
    let mut previous_num: u32 = 0;
    for char in chars {
        let digit = char.to_digit(10).unwrap();
        if digit < previous_num {
            decreasing = true;
            break;
        }
        previous_num = digit;
    }

    adjacent_nums && !decreasing
}

fn is_valid_part2(num: u32) -> bool {
    let chars = num.to_string().chars().collect::<Vec<char>>();
    // two adjacent digits are the same
    let mut adjacent_nums = false;
    for i in 0..chars.len()-1 {
        let left = if i > 0 {chars[i-1]} else {'a'};
        let pos1 = chars[i];
        let pos2 = chars[i+1];
        let right = if i+2 < chars.len() {chars[i+2]} else {'a'};

        if pos1 == pos2 && pos1 != left && pos2 != right {
            adjacent_nums = true;
            break;
        }
    }

    // decreasing
    let mut decreasing = false;
    let mut previous_num: u32 = 0;
    for char in chars {
        let digit = char.to_digit(10).unwrap();
        if digit < previous_num {
            decreasing = true;
            break;
        }
        previous_num = digit;
    }

    adjacent_nums && !decreasing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2814));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1991));
    }
}
