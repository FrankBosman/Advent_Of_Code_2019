use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let int_computer = IntCodeComputer::new(input);
    let result = int_computer.run(Vec::from(["1"]));
    Some((*result.unwrap().last().unwrap()) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let int_computer = IntCodeComputer::new(input);
    let result = int_computer.run(Vec::from(["5"]));
    Some((*result.unwrap().last().unwrap()) as u32)
}

struct IntCodeComputer {
    code: Vec<String>,
}

struct IntCode {
    opcode: u8,
    parameters: Vec<i32>,
    modes: Vec<usize>,
}

impl IntCodeComputer {
    pub(crate) fn new(input: &str) -> Self {
        let code = input.trim().split(",").map(|str| String::from(str)).collect::<Vec<String>>();
        Self { code }
    }

    pub(crate) fn run(&self, input_in: Vec<&str>) -> Option<Vec<i32>> {
        let mut output = vec![];
        let mut input = input_in.iter();
        let mut memory = self.code.clone();

        let mut instruction_pointer = 0;
        'run: while instruction_pointer < memory.len() {
            let parameter_count = IntCode::parameter_count(&memory[instruction_pointer]);
            let slice = &memory[instruction_pointer..(instruction_pointer + parameter_count + 1)];
            let int_code = match IntCode::parse(slice) {
                Some(int_code) => int_code,
                _ => panic!("Invalid operation: {:?}", slice),
            };

            let mut should_increase_pc = true;
            match int_code.opcode {
                1 => { // Addition
                    let value1 = int_code.get_parameter(0, &memory).unwrap();
                    let value2 = int_code.get_parameter(1, &memory).unwrap();
                    memory[int_code.parameter_addr(2)] = (value1 + value2).to_string();
                }
                2 => {
                    let value1 = int_code.get_parameter(0, &memory).unwrap();
                    let value2 = int_code.get_parameter(1, &memory).unwrap();
                    memory[int_code.parameter_addr(2)] = (value1 * value2).to_string();
                }
                3 => {  // Read input
                    memory[int_code.parameter_addr(0)] = input.next().unwrap().to_string();
                }
                4 => {  // Write to Output
                    let value: i32 = int_code.get_parameter(0, &memory).unwrap();
                    println!("{}", value);
                    output.push(value);
                }
                5 => {
                    let value1 = int_code.get_parameter(0, &memory).unwrap();
                    if value1 != 0 {
                        let value2 = int_code.get_parameter(1, &memory).unwrap();
                        should_increase_pc = false;
                        instruction_pointer = value2 as usize;
                    }
                }
                6 => {
                    let value1 = int_code.get_parameter(0, &memory).unwrap();
                    if value1 == 0 {
                        let value2 = int_code.get_parameter(1, &memory).unwrap();
                        should_increase_pc = false;
                        instruction_pointer = value2 as usize;
                    }
                }
                7 => {
                    let value1 = int_code.get_parameter(0, &memory).unwrap();
                    let value2 = int_code.get_parameter(1, &memory).unwrap();
                    memory[int_code.parameter_addr(2)] = if value1 < value2 { 1.to_string() } else { 0.to_string() };
                }
                8 => {
                    let value1 = int_code.get_parameter(0, &memory).unwrap();
                    let value2 = int_code.get_parameter(1, &memory).unwrap();
                    memory[int_code.parameter_addr(2)] = if value1 == value2 { 1.to_string() } else { 0.to_string() };
                }
                99 => break 'run,
                _ => panic!("Invalid or not implemented opcode: {}", int_code.opcode),
            }

            if should_increase_pc {
                instruction_pointer += parameter_count + 1;
            }
        }

        Some(output)
    }
}

impl IntCode {
    pub(crate) fn parse(slice: &[String]) -> Option<Self> {
        let mut operation = slice.iter();
        // Map with the amount of arguments for each instruction
        let instructions: HashMap<u8, usize> = IntCode::get_instructions();

        let mut op_iter = operation.next().unwrap().chars().rev();

        // Get the opcode
        let mut opcode_string = String::new();
        opcode_string.push(op_iter.next().unwrap_or('0'));
        opcode_string.insert(0, op_iter.next().unwrap_or('0'));

        // Parse opcode
        let opcode = match opcode_string.parse::<u8>() {
            Ok(opcode) => opcode,
            Err(_) => return None,
        };

        // Retrieve the amount of parameters for this opcode
        let parameters_len = match instructions.get(&opcode) {
            Some(&parameters_len) => parameters_len,
            None => panic!("IntCode: Unknown opcode: {}", opcode),
        };

        // Get the parameters and corresponding modes
        let mut modes: Vec<usize> = Vec::with_capacity(parameters_len);
        let mut parameters: Vec<i32> = Vec::with_capacity(parameters_len);
        for _i in 0..parameters_len {
            modes.push(op_iter.next().unwrap_or('0').to_digit(10)? as usize);
            parameters.push(operation.next().unwrap().parse::<i32>().unwrap());
        }

        // println!("IntCode: {}, {:?}, {:?}", opcode, parameters, modes);
        Some(IntCode::new(opcode, parameters, modes))
    }

    pub(crate) fn get_parameter(&self, index: usize, memory: &Vec<String>) -> Option<i32> {
        // Verify the index is valid
        if index >= self.parameters.len() {
            return None;
        }

        // Mode 0 is address pointer, mode 1 is direct value
        match self.modes[index] {
            0 => Some(memory.get(self.parameter_addr(index)).unwrap().parse::<i32>().unwrap()),
            1 => Some(*self.parameters.get(index).unwrap()),
            _ => None,
        }
    }

    pub(crate) fn parameter_addr(&self, index: usize) -> usize {
        (*self.parameters.get(index).unwrap()) as usize
    }

    pub(crate) fn parameter_count(op_string: &str) -> usize {
        // Map with the amount of arguments for each instruction
        let instructions: HashMap<u8, usize> = IntCode::get_instructions();
        let opcode = match IntCode::get_opcode(op_string) {
            Some(code) => code,
            None => panic!("Couldn't parse opcode, {}", op_string),
        };

        // Retrieve the amount of parameters for this opcode
        let parameters_len = match instructions.get(&opcode) {
            Some(&parameters_len) => parameters_len,
            None => panic!("IntCode: Unknown opcode: {}", opcode),
        };
        parameters_len
    }

    fn get_opcode(op_string: &str) -> Option<u8> {
        let mut op_iter = op_string.chars().rev();

        // Get the opcode
        let mut opcode_string = String::new();
        opcode_string.push(op_iter.next().unwrap_or('0'));
        opcode_string.insert(0, op_iter.next().unwrap_or('0'));

        // Parse opcode
        let opcode = match opcode_string.parse::<u8>() {
            Ok(opcode) => opcode,
            Err(_) => return None,
        };
        Some(opcode)
    }

    pub(crate) fn new(opcode: u8, parameters: Vec<i32>, modes: Vec<usize>) -> Self {
        Self { opcode, parameters, modes }
    }

    fn get_instructions() -> HashMap<u8, usize> {
        // Map with the amount of arguments for each instruction
        HashMap::from([(1, 3), (2, 3), (3, 1), (4, 1), (5, 2), (6, 2), (7, 3), (8, 3), (99, 0)])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(121));
    }
}
