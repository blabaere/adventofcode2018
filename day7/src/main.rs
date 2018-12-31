use std::str::FromStr;

#[macro_use]
extern crate nom;

mod model;

use self::model::*;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn get_requirements(lines: &[String]) -> Result<Vec<Requirement>, ()> {
    lines.iter().map(|line| FromStr::from_str(line)).collect()
}

fn get_input_lines(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    BufReader::new(file).lines().collect()
}

fn part1(instructions: &Instructions) {
    let steps = instructions.steps();
    let mut answer = String::new();

    for step in steps {
        answer.push(step);
    }

    println!("Part1: '{}'", answer);
}

fn part2(instructions: &Instructions) {
    let steps = instructions.steps();
    let mut team = Team::new(5, true);
    let time = team.complete_steps(steps);

    println!("Part2: '{}'", time);
}

fn main() -> io::Result<()> {
    let os_args: Vec<_> = std::env::args().collect();
    let lines = get_input_lines(&os_args[1])?;
    let requirements = get_requirements(&lines).unwrap();
    let instructions = Instructions::new(requirements);

    part1(&instructions);
    part2(&instructions);

    Ok(())
}
