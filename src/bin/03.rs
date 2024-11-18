use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub(crate) fn get_points(&self, input: &str) -> HashSet<Point> {
        let direction = input.chars().next().unwrap();
        let mut chars = input.chars();
        chars.next();
        let distance = chars.as_str().parse::<u32>().unwrap();
        match direction {
            'R' => (1..=distance).map(|i| Point {x: self.x + i as i32, y: self.y}).collect::<HashSet<Point>>(),
            'L' => (1..=distance).map(|i| Point {x: self.x - i as i32, y: self.y}).collect::<HashSet<Point>>(),
            'D' => (1..=distance).map(|i| Point {x: self.x, y: self.y + i as i32}).collect::<HashSet<Point>>(),
            'U' => (1..=distance).map(|i| Point {x: self.x, y: self.y - i as i32}).collect::<HashSet<Point>>(),
            _ => panic!("Unknown direction: {}", direction),
        }
    }
    pub(crate) fn get_costs(&self, input: &str, cost: &u32) -> Vec<(Point, u32)> {
        let direction = input.chars().next().unwrap();
        let mut chars = input.chars();
        chars.next();
        let distance = chars.as_str().parse::<u32>().unwrap();
        match direction {
            'R' => (1..=distance).map(|i| (Point {x: self.x + i as i32, y: self.y}, cost + i)).collect::<Vec<(Point, u32)>>(),
            'L' => (1..=distance).map(|i| (Point {x: self.x - i as i32, y: self.y}, cost + i)).collect::<Vec<(Point, u32)>>(),
            'D' => (1..=distance).map(|i| (Point {x: self.x, y: self.y + i as i32}, cost + i)).collect::<Vec<(Point, u32)>>(),
            'U' => (1..=distance).map(|i| (Point {x: self.x, y: self.y - i as i32}, cost + i)).collect::<Vec<(Point, u32)>>(),
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    pub(crate) fn update(&self, input: &str, cost: &u32) -> (Point, u32) {
        let direction = input.chars().next().unwrap();
        let mut chars = input.chars();
        chars.next();
        let distance = chars.as_str().parse::<i32>().unwrap();
        match direction {
            'R' => (Point {x: self.x + distance, y: self.y}, cost + distance.abs() as u32),
            'L' => (Point {x: self.x - distance, y: self.y}, cost + distance.abs() as u32),
            'D' => (Point {x: self.x, y: self.y + distance}, cost + distance.abs() as u32),
            'U' => (Point {x: self.x, y: self.y - distance}, cost + distance.abs() as u32),
            _ => panic!("Unknown direction: {}", direction),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim().lines().collect::<Vec<_>>();
    let (path1, _cost1) = create_path(lines[0]);
    let (path2, _cost2) = create_path(lines[1]);

    let intersections = path1.intersection(&path2);

    let mut shortest_distance = i32::MAX;
    for point in intersections {
        let distance = point.x.abs() + point.y.abs();
        if distance < shortest_distance {
            shortest_distance = distance;
        }
    }

    Some(shortest_distance as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim().lines().collect::<Vec<_>>();
    let (path1, cost1) = create_path(lines[0]);
    let (path2, cost2) = create_path(lines[1]);

    let intersections = path1.intersection(&path2);

    let mut fewest_steps = u32::MAX;
    for point in intersections {
        let (steps1, steps2) = (cost1.get(point).unwrap(), cost2.get(point).unwrap());
        let steps = steps1 + steps2;
        if steps < fewest_steps {
            fewest_steps = steps;
        }
    }

    Some(fewest_steps)
}

fn create_path(line: &str) -> (HashSet<Point>, HashMap<Point, u32>) {
    let mut path = HashSet::new();
    let mut costs = HashMap::new();
    let directions = line.split(",").collect::<Vec<_>>();

    let mut point = Point { x: 0, y: 0 };
    let mut cost = 0u32;
    for direction in directions {
        path.extend(point.get_points(direction));
        let new_costs = point.get_costs(direction, &cost);

        // Add all the new costs, costs that are already in the map have to be lower.
        for (new_point, new_cost) in new_costs {
            if !costs.contains_key(&new_point) {
                costs.insert(new_point, new_cost);
            }
        }

        (point, cost) = point.update(direction, &cost);
    }
    (path, costs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(135));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(410));
    }
}
