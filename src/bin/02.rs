advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut code = load(input);
    let result = run(&mut code, 12, 2);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let wanted_result = 19690720;
    let code = load(input);

    for noun in 0..=99 {
        for verb in 0..=99 {
            if run(&mut code.clone(), noun, verb) == wanted_result {
                return Some(100 * noun + verb);
            }
        }
    }

    None
}

fn load(input: &str) -> Vec<u32> {
    input.trim().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()
}

fn run(code: &mut Vec<u32>, noun: u32, verb: u32) -> u32 {
    code[1] = noun;
    code[2] = verb;
    'main: for i in (0..code.len()).step_by(4) {
        let op_code = code.get(i).unwrap();
        let pos = &code[i + 1..i + 4].to_vec();

        match op_code {
            &1 => code[pos[2] as usize] = code[pos[0] as usize] + code[pos[1] as usize],
            &2 => code[pos[2] as usize] = code[pos[0] as usize] * code[pos[1] as usize],
            &99 => {
                break 'main
            }
            _ => panic!("Unknown opCode {}", op_code)
        }
    }
    code[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1202));
    }
}
