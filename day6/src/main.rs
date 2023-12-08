
fn main() {
    let races = Vec::<Race>::from_file("data/input.txt").unwrap();
    let race  = Race::from_file("data/input.txt").unwrap();
    println!("Count: {}", count_possible_wins(&races));
    println!("Ways: {}", find_range(&race).unwrap().count());
}

#[test]
fn test_product_all_possible_winning_times() {
    let races = Vec::<Race>::from_file("data/test.txt").unwrap();
    assert_eq!(count_possible_wins(&races), 288);
}

#[test]
fn test_number_of_ways_to_beat_kerning(){
    let  race = Race::from_file("data/test.txt").unwrap();
    assert_eq!(find_range(&race).unwrap().count(), 71503);
}

struct Race {
    best_time: Time,
    distance: Distance,
}

type Time = Number;
type Distance = Number;
type Number = f64;



impl<I> FromFile for I
where
    I: FromIterator<Race>,
{
    type Error = anyhow::Error;
    fn from_file(path: &str) -> anyhow::Result<I> {
        let input = std::fs::read_to_string(path)?;

        let mut iter = input.lines();
        let (best_times_str, distances_str): (&str, &str) =
            (iter.next().unwrap(), iter.next().unwrap());
        let best_times = best_times_str
            .strip_prefix("Time:")
            .ok_or(anyhow::anyhow!(
                "Failed to parse the file: \"Time:\" marker not found."
            ))?
            .trim()
            .split_whitespace()
            .map(|number| number.parse::<Time>().unwrap())
            .into_iter();
        let distances = distances_str
            .strip_prefix("Distance:")
            .ok_or(anyhow::anyhow!(
                "Failed to parse the file: \"Distance:\" marker not found."
            ))?
            .trim()
            .split_whitespace()
            .map(|number| number.parse::<Distance>().unwrap())
            .into_iter();

        Ok(best_times
            .zip(distances)
            .map(|(best_time, distance)| Race {
                best_time,
                distance,
            })
            .collect())
    }
}

impl FromFile for Race{ 
    type Error = anyhow::Error;
    fn from_file(path: &str) -> Result<Self, Self::Error> {
        let input = std::fs::read_to_string(path)?;
        let mut lines = input.lines();
        let time = lines.next().unwrap().trim_start_matches("Time:").replace(' ', "").trim().parse::<Time>().unwrap();
        let distance = lines.next().unwrap().trim_start_matches("Distance:").replace(' ',"").trim().parse::<Distance>().unwrap();
        Ok(Race { best_time: time, distance})
    }
}

trait FromFile: Sized {
    type Error;
    fn from_file(path: &str) -> Result<Self, Self::Error>;
}

fn count_possible_wins(races: &[Race]) -> usize {
    races
        .iter()
        .filter_map(|race| find_range(race))
        .map(|range| range.count())
        .product()
}

fn find_range(race: &Race) -> Option<std::ops::Range<u32>> {
    let delta = race.best_time * race.best_time - 4f64 * race.distance;
    if delta <= 0.0f64 {
        return None;
    }

    let x1 = (race.best_time + delta.sqrt()) / 2f64;
    let x2 = (race.best_time - delta.sqrt()) / 2f64;
    let start_range = x2.floor() as u32 + 1;
    let end_range = x1.ceil() as u32;

    Some(start_range..end_range)
}





