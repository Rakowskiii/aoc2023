use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use indicatif::{ProgressBar, ProgressStyle};
use num::integer::lcm;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    let (path, map) = input.split_once('\n').unwrap();
    let map = Navigation::<Human>::parse_map(map.trim());
    println!("Path length is: {}", Navigation::<Human>::find_path_length(path, &map));
    println!("Shortest path leading all ghosts length is: {}", Navigation::<Ghost>::find_path_length(path, &map));
}


static PATHS: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"(?<src>[A-Z0-9]{3}).*(?<dstl>[A-Z0-9]{3}).*(?<dstr>[A-Z0-9]{3})").unwrap());

#[test]
fn test_find_path_length() {
    let input = std::fs::read_to_string("data/test.txt").unwrap();
    let (path, map) = input.split_once('\n').unwrap();
    let map = Navigation::<Human>::parse_map(map.trim());

    assert_eq!(Navigation::<Human>::find_path_length(path, &map), 2);
}

#[test]
fn test_find_ghost_path() {
    let input = std::fs::read_to_string("data/test_ghost.txt").unwrap();
    let (path, map) = input.split_once('\n').unwrap();
    let map = Navigation::<Human>::parse_map(map.trim());

    assert_eq!(Navigation::<Ghost>::find_path_length(path, &map), 6);
}

struct Navigation<T> {
    _phantom: PhantomData<T>,
}

type Place = String;
type Map = std::collections::HashMap<String, Cross>;

struct Human;

struct Ghost;


impl <T>Navigation<T> {
    fn parse_map(input: &str) -> Map {
        input.lines().map(|line| Self::parse_crosses(line)).collect()
    }
    fn parse_crosses(line: &str) -> (Place, Cross) {
        let paths = PATHS.captures(line).unwrap();
        (paths["src"].to_string(), Cross {
            left_path: paths["dstl"].to_string(),
            right_path: paths["dstr"].to_string(),
        })
    }
}

impl Navigation<Human> {
    fn find_path_length(path: &str, map: &Map) -> u32 {
        let mut len = 0;
        let mut current_location = "AAA";

        loop {
            for direction in path.chars() {
                match direction {
                    'R' => current_location = map.get(current_location).unwrap().right_path.as_str(),
                    'L' => current_location = map.get(current_location).unwrap().left_path.as_str(),
                    _ => { dbg!("What?"); }
                }
                len += 1;
                if current_location == "ZZZ" { return len; }
            }
        }
    }
}


impl Navigation<Ghost> {
    fn find_path_length(path: &str, map: &Map) -> u64 {
        let start_positions: Vec<String> = map.keys().cloned().filter(|position| position.ends_with('A')).collect();


        let progress = Arc::new(AtomicUsize::new(0));
        let total = start_positions.len();
        let pb = ProgressBar::new(total as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .unwrap().progress_chars("#>-"));

        let res: Vec<u64> = start_positions
            .into_par_iter()
            .map_with(progress.clone(), |progress, mut current_location| {
                let mut len: u64 = 0;
                let mut peroid = 0;
                let mut found = false;
                loop {
                    for direction in path.chars() {
                        match direction {
                            'R' => current_location = map.get(&current_location).unwrap().right_path.clone(),
                            'L' => current_location = map.get(&current_location).unwrap().left_path.clone(),
                            _ => { dbg!("What?"); }
                        }
                        if !found { len += 1 } else { peroid += 1 };
                        if current_location.ends_with('Z') {
                            if !found { found = true } else {
                                progress.fetch_add(1, Ordering::SeqCst);
                                pb.inc(1); // Update progress bar
                                // return (len, peroid);
                                // All paths turned out to be periodic
                                return len;
                            }
                        }
                    }
                }
            }).collect();


        res.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
    }
}

#[derive(Debug)]
struct Cross {
    left_path: String,
    right_path: String,
}

