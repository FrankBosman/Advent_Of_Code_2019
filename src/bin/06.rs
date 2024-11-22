use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

advent_of_code::solution!(6);

#[derive(Debug)]
struct CelestialBody {
    name: String,
    moons: Vec<Rc<RefCell<CelestialBody>>>,
    parent: Option<Weak<RefCell<CelestialBody>>>,
    path_length: u32,
}
impl CelestialBody {
    fn new(name: String, parent: Option<Weak<RefCell<CelestialBody>>>, path_length: u32) -> Self {
        Self { name, moons: vec![], parent, path_length }
    }

    fn add_moons(&mut self, moon_names: Option<&Vec<&str>>, self_ref: Weak<RefCell<CelestialBody>>, map: &HashMap<&str, Vec<&str>>) -> u32 {
        if moon_names.is_none() { return self.path_length; }
        let moon_names = moon_names.unwrap();

        let mut new_moons = Vec::with_capacity(moon_names.len());
        let mut sum = 0u32;
        for &name in moon_names {
            let moon_ref = Rc::new(RefCell::new(CelestialBody::new(name.to_string(), Some(self_ref.clone()), self.path_length + 1)));
            let weak_ref = Rc::downgrade(&moon_ref);
            new_moons.push(moon_ref);
            let mut moon: RefMut<'_, _> = new_moons.last_mut().unwrap().borrow_mut();
            sum += moon.add_moons(map.get(name), weak_ref, map);
        }

        self.moons.extend(new_moons);
        sum + self.path_length
    }

    fn find(&self, target: &str) -> Option<Rc<RefCell<CelestialBody>>> {
        for moon_ref in self.moons.iter() {
            let moon = moon_ref.borrow();
            if moon.name == target {
                return Some(moon_ref.clone());
            }

            let result = moon.find(target);
            if result.is_some() {
                return result;
            }
        }
        None
    }

    fn traverse(&self, target: &str, distance: u32, from: &str) -> Option<u32> {
        // Group all the connected nodes
        let mut connected = vec![];
        if let Some(parent) = &self.parent { connected.push(parent.upgrade().unwrap()) }
        connected.extend(self.moons.iter().map(|moon| moon.clone()));

        // Loop over all connected nodes and traverse the solar system
        for next_ref in connected {
            let next = match next_ref.try_borrow() {
                Ok(result) => result,
                Err(_) => continue
            };
            if next.name == from { continue; }  // Do not go back to the node we came from

            // If the target is found, return it
            if next.name == target { return Some(distance); }

            // Traverse depth first, if the target is found, return the distance
            let result = next.traverse(target, distance + 1, self.name.as_str());
            if result.is_some() { return result; }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = create_map(input);

    // Create Linked list
    let root_ref = Rc::new(RefCell::new(CelestialBody::new("COM".into(), None, 0)));
    let mut root: RefMut<'_, _> = root_ref.borrow_mut();
    let result = root.add_moons(map.get("COM"), Rc::downgrade(&root_ref), &map);

    // println!("{:#?}", root);
    // solve_1_simple(&map)
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = create_map(input);

    // Create Linked list
    let root_ref = Rc::new(RefCell::new(CelestialBody::new("COM".into(), None, 0)));
    let mut root: RefMut<'_, _> = root_ref.borrow_mut();
    let _result = root.add_moons(map.get("COM"), Rc::downgrade(&root_ref), &map);

    // Get the start node and traverse to the end node
    let you_ref = root.find("YOU").unwrap();
    let you_node = you_ref.borrow();
    Some(you_node.traverse("SAN", 0, "").unwrap() - 1u32)
}
fn create_map(input: &str) -> HashMap<&str, Vec<&str>> {
    let lines = input.lines().collect::<Vec<_>>();

    // Create a map of the solar system
    let mut map: HashMap<&str, Vec<&str>> = HashMap::with_capacity(lines.len());
    for line in lines {
        let (part1, part2) = line.split_once(")").unwrap();

        match map.get_mut(part1) {
            None => { map.insert(part1, vec![part2]); }
            Some(result) => result.push(part2),
        }
    }
    map
}

fn solve_1_simple(map: &HashMap<&str, Vec<&str>>) -> Option<u32> {
    let mut frontier = vec![(map.get("COM").unwrap(), 0u32)];
    let mut result = 0u32;
    loop {
        let (children, path_length) = frontier.pop().unwrap();
        result += path_length;

        for &child in children {
            match map.get(child) {
                None => result += path_length + 1,  // Add leave node
                Some(result) => frontier.push((result, path_length + 1)),
            }
        }

        if frontier.is_empty() { break; }
    }
    Some(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
