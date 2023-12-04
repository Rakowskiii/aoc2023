use std::io::BufRead;

fn main() -> anyhow::Result<()>{
    let inputs = std::fs::read_to_string("./data/input.txt")?;
    println!("The calibration is: {}", calibration(&inputs));
    println!("The numberized calibration is: {}", calibration(&numberize(inputs)));
    Ok(())
}

#[test]
fn test_calibration() -> anyhow::Result<()>{
    let inputs = std::fs::read_to_string("data/test_input.txt")?;
    dbg!(calibration(&inputs));
    Ok(())
}

#[test]
fn test_calibration_spelled() -> anyhow::Result<()>{
    let inputs = std::fs::read_to_string("data/test_input_spelled.txt")?;
    dbg!(calibration(&numberize(inputs)));
    Ok(())
}

fn calibration(document: &str) -> u32 {
    document.lines().map(|line| {
        let mut iter = line.chars().filter_map(|c| c.to_digit(10));
        let first = iter.next();
        let last = iter.last().or(first);
        // We are guaranteed at least one digit in each line
        // So first is always Some
       first.unwrap()*10 + last.unwrap()
    }).reduce(|a,b| a+b).unwrap()
}

fn numberize(spelled: String) -> String {
    spelled
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero")
}