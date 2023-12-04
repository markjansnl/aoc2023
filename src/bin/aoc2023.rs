use aoc2023::prelude::*;

use chrono::{Datelike, Local};
use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Output format. If ommitted, you can only run one day/example. If no day is specified, today is run.
    #[arg(short, long)]
    format: Option<Format>,

    /// Day to run. Repeat for more days. If this and --today are omitted, all days are run.
    #[arg(short, long, num_args = 0.., value_delimiter = ',')]
    day: Vec<u8>,

    /// Run today
    #[arg(long)]
    today: bool,

    /// Run a single example. Repeat to run more examples.
    #[arg(short, long, value_parser, num_args = 0.., value_delimiter = ',')]
    example: Option<Vec<usize>>,

    /// Run only one part. If both parts are run, the parsed input is reused if possible.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    /// Output results in JSON format
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut outputs: Vec<(u8, usize, (Option<String>, Option<String>))> = Vec::new();

    let days: Vec<_> = if cli.format.is_some() {
        if cli.today {
            DAYS.iter()
                .filter(|d| d.day == Local::now().day() as u8)
                .collect()
        } else if cli.day.is_empty() {
            DAYS.iter().collect()
        } else {
            DAYS.iter().filter(|d| cli.day.contains(&d.day)).collect()
        }
    } else if cli.today || cli.day.is_empty() {
        DAYS.iter()
            .filter(|d| d.day == Local::now().day() as u8)
            .collect()
    } else {
        DAYS.iter().filter(|d| d.day == cli.day[0]).collect()
    };

    for day in days {
        if let Some(examples) = &cli.example {
            let examples: Vec<_> = if cli.format.is_none() {
                if examples.is_empty() {
                    day.examples.iter().take(1).collect()
                } else {
                    day.examples
                        .iter()
                        .filter(|example| example.example == examples[0])
                        .collect()
                }
            } else if examples.is_empty() {
                day.examples.iter().collect()
            } else {
                day.examples
                    .iter()
                    .filter(|example| examples.contains(&example.example))
                    .collect()
            };

            for example in examples {
                outputs.push((
                    day.day,
                    example.example,
                    (run_day(
                        day.day,
                        get_input(day.day, example.example)?,
                        cli.part.is_none() || cli.part.unwrap() == 1,
                        cli.part.is_none() || cli.part.unwrap() == 2,
                    )?),
                ));
            }
        } else {
            outputs.push((
                day.day,
                0,
                (run_day(
                    day.day,
                    get_input(day.day, 0)?,
                    cli.part.is_none() || cli.part.unwrap() == 1,
                    cli.part.is_none() || cli.part.unwrap() == 2,
                )?),
            ));
        }
    }

    if cli.format.is_none() {
        for (_, _, (part1, part2)) in outputs {
            if let Some(output) = part1 {
                println!("{output}");
            }
            if let Some(output) = part2 {
                println!("{output}");
            }
        }
    } else {
        let json = serde_json::to_string(
            &outputs
                .iter()
                .map(|(day, example, (part1, part2))| {
                    let mut vec = Vec::new();
                    if let Some(output) = part1 {
                        vec.push(JsonOutput {
                            day: *day,
                            example: if example == &0 { None } else { Some(*example) },
                            part: 1,
                            output: output.clone(),
                        });
                    }
                    if let Some(output) = part2 {
                        vec.push(JsonOutput {
                            day: *day,
                            example: if example == &0 { None } else { Some(*example) },
                            part: 2,
                            output: output.clone(),
                        });
                    }
                    vec
                })
                .flatten()
                .collect::<Vec<_>>(),
        )?;

        println!("{json}");
    }

    Ok(())
}

#[derive(Serialize)]
struct JsonOutput {
    day: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<usize>,
    part: u8,
    output: String,
}
