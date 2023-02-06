// SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{arg, command, value_parser};
use std::env;

use fptbs::iterative;
use fptbs::optimized;

fn main() {
    let matches = command!()
        .arg(
            arg!([sum] "the target sum (must be even)")
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!([mode] "search mode")
                .required(false)
                .value_parser(["optimized", "iterative"])
                .default_value("optimized"),
        )
        .get_matches();

    let sum = matches.get_one::<u64>("sum").unwrap();
    if sum % 2 != 0 {
        eprintln!("Error: The target sum must be even");
        std::process::exit(1)
    }
    if *sum > (u64::MAX as f64 + 1.0).sqrt() as u64 {
        eprintln!("Error: Overflow reached, try a lower sum");
        std::process::exit(1)
    }
    let mode = matches.get_one::<String>("mode").unwrap();

    match mode as &str {
        "iterative" => println!("{}", iterative::find(*sum)),
        "optimized" => println!("{}", optimized::find(*sum)),
        _ => println!("ERROR"),
    }
}
