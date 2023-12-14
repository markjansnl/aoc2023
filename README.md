# Advent of Code 2023

In this repository you can find the source code of my solutions to the [Advent of Code 2023](https://adventofcode.com/2023) puzzles.

Every day is implemented in a struct `DayXX`, which implements trait `Day`. A [template](template) is available.

Configuration is done in [days.rs](src/days.rs). Macros will make the day implementations available for:
* [Command Line Interface](#command-line-interface)
* [Test Cases](#test-cases)
* [Benchmarks](#benchmarks)

Have fun looking at the source code and/or give it a try with your own input. I had a lot of fun implementing this and I'm proud of the result.

With kind regards,<br>
Mark Jans

## Command Line Interface

To run the [aoc2023.rs](src/bin/aoc2023.rs) Command Line Interface, call `cargo run [--release] [-- <PARAMETERS>]`. The following `<PARAMETERS>` are accepted:
* `--format json`:          Run all days or all examples if not filtered with another parameter, and output in JSON format.
* `--day [<DAYS>]`:         Run only days `<DAYS>`. For JSON output a comma separated list can be provided. For normal output
                            only the first provided day will be run. If `<DAY>` is omitted, the current day will be run.
* `--part <PART>`:          Run only part `<PART>`.
* `--example [<EXAMPLES>]`  Run example inputs instead. For JSON output a comma separated list of examples can be provied. For
                            plain text input only the first provided example number will be run. If `<EXAMPLES>` is ommitted, all
                            examples will be run for JSON output, and today for plain text output.

## Test Cases

The expected answers per example per day and day part can be configured in [days.rs](src/days.rs). Run `cargo test` to run all test cases.

You can also run some of the testcases:
* `cargo test days::tests::dayXX` to run all test cases for dayXX.
* `cargo test days::tests::dayXX::exampleX` to run all test cases for dayXX exampleX.
* `cargo test days::tests::dayXX::exampleX::partX` to run the test case for dayXX partX exampleX.

## Benchmarks

All days can be benchmarked using Criterion. Run `cargo bench` to run all benchmarks on your own system.
To run the benchmarks for a single day run `cargo bench 'Day XX'`.

The sample size can be configured in [days.rs](src/days.rs).

Some nice charts are generated. You can find them after benchmarking in `target/criterion/report/index.html`.
