advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line|line.parse::<u32>().unwrap() / 3u32 - 2u32).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| calc_fuel(line.parse::<u32>().unwrap())).sum::<u32>())
}

fn calc_fuel(mass: u32) -> u32 {
    let mut total = 0;
    let mut weight = mass;

    while weight > 0 {
        let mut fuel = weight / 3;
        if fuel > 2 {
            fuel -= 2;
        } else {
            break;
        }
        total += fuel;
        weight = fuel;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
