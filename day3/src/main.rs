// This code is solving a problem with reduced scope, we can be sure here that
// those will never happen, as we know the inputs. This allows us to keep code more concise
// focusing only on solving the problem
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
use std::collections::{HashMap, HashSet};

fn main() {
    let mut processor = EngineProcessor::new_from_file("data/input.txt");
    processor.parse_parts();
    println!(
        "Found the sum of engine part numbers: {}",
        processor.get_engine_sum()
    );
    println!(
        "Found the sum of gear products: {}",
        processor.get_gear_sum()
    );
}

#[test]
fn sum_part_numbers() {
    let mut processor = EngineProcessor::new_from_file("data/test.txt");
    processor.parse_parts();
    dbg!(&processor);
    assert_eq!(processor.get_engine_sum(), 4361);
}

#[test]
fn sum_gear_ratios() {
    let mut processor = EngineProcessor::new_from_file("data/test.txt");
    processor.parse_parts();
    dbg!(processor.get_gear_sum());
}

type PartId = u32;

#[derive(Debug)]
struct EngineProcessor {
    data: Vec<Vec<char>>,
    // change to option so that you cant call get sum with no
    parts: HashMap<u32, Part>,
    gears: HashMap<Position, HashSet<PartId>>,
}

#[derive(Debug, Default, Clone)]
struct Part {
    digits: String,
    neighbored: bool,
}

impl Part {
    fn new() -> Self {
        Self::default()
    }

    fn append(&mut self, ch: char) {
        self.digits.push(ch);
    }
}

impl EngineProcessor {
    const NEIGHBORHOOD: [(i16, i16); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    fn new_from_file(input: &str) -> Self {
        Self {
            data: Self::load_inputs(input)
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
            parts: HashMap::new(),
            gears: HashMap::new(),
        }
    }

    fn load_inputs(path: &str) -> String {
        std::fs::read_to_string(path).unwrap()
    }

    fn parse_parts(&mut self) {
        let mut part = Part::new();
        let mut part_id = 0;

        // Check each symbol
        for (y, line) in self.data.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if char.is_ascii_digit() {
                    part.append(*char);
                    if let Some(neighbors) = self.get_neighbors(x, y) {
                        part.neighbored = true;
                        neighbors
                            .iter()
                            .filter(|neighbor| neighbor.symbol == '*')
                            .for_each(|neighbor| {
                                self.gears
                                    .entry(neighbor.position)
                                    .or_default()
                                    .insert(part_id);
                            });
                    }
                } else if !part.digits.is_empty() {
                    self.parts.insert(part_id, part.clone());
                    part = Part::new();
                    part_id += 1;
                }
            }
        }

        // Clean gears of those, that did not have two separate parts touching them
        self.gears.retain(|_, set| set.len() == 2);
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Option<Vec<Neighbor>> {
        let mut neighbors = Vec::new();
        for (off_x, off_y) in &Self::NEIGHBORHOOD {
            let nx = x as i16 + off_x;
            let ny = y as i16 + off_y;
            if nx < 0
                || ny < 0
                || ny as usize >= self.data.len()
                || nx as usize >= self.data[ny as usize].len()
            {
                continue;
            }
            let char = self.data[ny as usize][nx as usize];
            if !char.is_numeric() && char != '.' {
                neighbors.push(Neighbor {
                    symbol: char,
                    position: (nx as usize, ny as usize),
                });
            }
        }
        if neighbors.is_empty() {
            return None;
        }
        Some(neighbors)
    }

    fn get_engine_sum(&self) -> u32 {
        self.parts
            .iter()
            .filter_map(|(_, part)| if part.neighbored { part.digits.parse::<u32>().ok() } else { None })
            .sum()
    }

    fn get_part_digits(&self, part_id: &PartId) -> Option<u32> {
        self.parts.get( part_id).and_then(|part| part.digits.parse::<u32>().ok())
    }

    fn get_gear_sum(&self) -> u32{
        let find_gears_values = |ids: &HashSet<u32>| {
            ids.iter()
                .filter_map(|id| self.get_part_digits(id))
                .fold(1, |x, acc| x*acc)
        };

        self.gears
            .values()
            .map(find_gears_values)
            .sum()
    }
}

struct Neighbor {
    symbol: char,
    position: Position,
}

type Position = (usize,usize);
