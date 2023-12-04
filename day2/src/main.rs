use std::marker::PhantomData;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("data/input.txt")?;
    let mut game_verifier = GameProcessor::<Verifier>::new(12, 13, 14);

    println!(
        "Sum of valid games' ids is: {}",
        game_verifier.process(&input)
    );

    let mut game_counter = GameProcessor::<Counter>::new();
    println!("Sum of games' powers is: {}", game_counter.process(&input));

    Ok(())
}

#[test]
fn game_possible_id_sum() {
    let mut gv = GameProcessor::<Verifier>::new(12, 13, 14);
    let input = std::fs::read_to_string("data/test.txt").unwrap();
    assert_eq!(gv.process(&input), 8);
}

#[test]
fn sum_of_game_powers() {
    let mut gc = GameProcessor::<Counter>::new();
    let input = std::fs::read_to_string("data/test.txt").unwrap();
    assert_eq!(gc.process(&input), 2286);
}

#[derive(Default, Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::Blue),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            _ => Err(anyhow::anyhow!("Color not recognized")),
        }
    }
}

impl FromStr for Draw {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut draw = Draw::default();
        for part in s.split(',') {
            let mut subparts = part.split_whitespace();
            let amount = subparts
                .next()
                .ok_or(anyhow::anyhow!("Invalid amount"))?
                .parse::<u32>()?;
            let color = Color::from_str(subparts.next().ok_or(anyhow::anyhow!("Invalid color"))?)?;

            match color {
                Color::Green => draw.green += amount,
                Color::Blue => draw.blue += amount,
                Color::Red => draw.red += amount,
            }
        }
        Ok(draw)
    }
}

type GameId = u32;

#[derive(Debug)]
struct Game {
    id: GameId,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let game_id = split
            .next()
            .ok_or(anyhow::anyhow!("Failed to parse game"))?
            .split_whitespace()
            .last()
            .ok_or(anyhow::anyhow!("Failed to parse game_id"))?
            .parse::<GameId>()?;

        let mut game = Self {
            id: game_id,
            draws: vec![],
        };

        for draw in split
            .next()
            .ok_or(anyhow::anyhow!("Failed to parse draws"))?
            .split(';')
        {
            game.draws.push(Draw::from_str(draw)?);
        }
        Ok(game)
    }
}

struct Verifier;

#[derive(Default, Debug)]
struct Counter;
trait Processor {
    fn process(&mut self, input: &str) -> u32;
}

#[derive(Default, Debug)]
struct GameProcessor<T> {
    max_red: u32,
    max_blue: u32,
    max_green: u32,
    _phantom: PhantomData<T>,
}

impl GameProcessor<Verifier> {
    fn new(max_red: u32, max_green: u32, max_blue: u32) -> Self {
        Self {
            max_blue,
            max_green,
            max_red,
            _phantom: PhantomData,
        }
    }

    fn is_game_valid(&self, game: &Game) -> bool {
        for draw in &game.draws {
            if !self.is_draw_valid(draw) {
                return false;
            }
        }
        true
    }
    fn is_draw_valid(&self, draw: &Draw) -> bool {
        draw.blue <= self.max_blue && draw.red <= self.max_red && draw.green <= self.max_green
    }
}

impl Processor for GameProcessor<Verifier> {
    fn process(&mut self, input: &str) -> u32 {
        input
            .lines()
            .filter_map(|line| {
                Game::from_str(line)
                    .ok()
                    .filter(|game| self.is_game_valid(game))
                    .map(|game| game.id)
            })
            .sum()
    }
}

impl GameProcessor<Counter> {
    fn new() -> Self {
        Self::default()
    }

    fn set_max_for_game(&mut self, game: &Game) {
        for draw in &game.draws {
            if draw.green > self.max_green {
                self.max_green = draw.green;
            }
            if draw.blue > self.max_blue {
                self.max_blue = draw.blue;
            }
            if draw.red > self.max_red {
                self.max_red = draw.red;
            }
        }
    }

    fn yield_power(&mut self) -> u32 {
        let power = self.max_blue * self.max_green * self.max_red;
        (self.max_red, self.max_green, self.max_blue) = (0, 0, 0);
        power
    }
}

impl Processor for GameProcessor<Counter> {
    fn process(&mut self, input: &str) -> u32 {
        input
            .lines()
            .filter_map(|line| {
                Game::from_str(line).ok().map(|game| {
                    self.set_max_for_game(&game);
                    self.yield_power()
                })
            })
            .sum()
    }
}
