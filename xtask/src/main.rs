//! # local cargo-xtask
//!
//! It's unclear to me that I have a use for this currently.
//! It overlaps with `justfile` functionality.
//! But, while it offers strengths in complex custom cases,
//! it it less quickly legible when the justfile commands are
//! almost entirely series of shell command calls.
//!
//! Calling shells commands from rust doesn't simplify the
//! call logic.  And doesn't seem like it would be abile to
//! assist with fragility much.
//!
//! Only in cases where I had complex behavior would xtask feel
//! like it brought something to the table. (or, perhaps, collaboration)
//!
//! Note: as long as  (1)`xtask/` is displayed prominently in root
//!       and (2) a clap interface with command descriptions is present
//!       then basic command discoverability should be on par with just
mod types_manual;

use clap::Parser;
use owo_colors::{self as _, OwoColorize};

use crate::types_manual::*;

/// xtasks, repo convenience tasks
#[derive(Parser, Debug)]
#[command(version, about, long_about, disable_help_subcommand = true, subcommand_help_heading = "input source")]
enum Args {
        /// say hello
        Hello,
        /// add two numbers
        Add { a: i32, b: i32 },
        /// List prime components of a rust std type
        // #[arg[(value_enum = "TypesManual")]]
        Primes { t: TypesManual },
}

fn main() {
        match Args::parse() {
                Args::Hello => println!("Hello, world"),
                Args::Add { a, b } => {
                        let sum = a + b;
                        let sum = sum.green();
                        let a = a.red();
                        let b = b.blue();
                        println!("The (hex) sum of {a:>16x}  and {b:>16x} is {sum:>16x}");
                        println!("The (dec) sum of {a:>16}  and {b:>16} is {sum:>16}");
                        println!("The (oct) sum of {a:>16o}  and {b:>16o} is {sum:>16o}");
                        println!("The (bin) sum of {a:>16b}  and {b:>16b} is {sum:>16b}");
                }
                Args::Primes { t } => {
                        let t_deets = t.get_details_as_strings();
                        println!("{:?}", t_deets);
                        type TForPrimes = usize;
                        let upper_bound = match t_deets.max.parse::<TForPrimes>() {
                                Ok(n) => {
                                        if n <= 10_000_000 {
                                                n
                                        } else {
                                                eprintln!(
                                                        "{}'s max value ({}) will take a long time for us to calculate with the current method.",
                                                        t_deets.name, t_deets.max,
                                                );
                                                eprintln!(
                                                        "We're going to skip prime calculation. And yes, alas, this rules out any i or u smaller than _16."
                                                );
                                                return;
                                        }
                                }
                                Err(e) => {
                                        eprintln!(
                                                "Error parsing {}'s max value ({}) as {}: {}",
                                                t_deets.name,
                                                t_deets.max,
                                                std::any::type_name::<TForPrimes>(),
                                                e
                                        );
                                        eprintln!(
                                                "Note: type resilient implementation has not yet been ... um, implemented."
                                        );
                                        return;
                                }
                        };
                        let lower_bound = None;
                        let found_primes = prime_sieve(lower_bound, upper_bound);
                        println!("Number of primes found <= {}: {}", upper_bound, found_primes.len());
                        println!(
                                "which makes the range ({}..={}) {:.1}% prime.",
                                0, // lower_bound.unwrap_or(0),
                                upper_bound,
                                100. * (found_primes.len() as f32) / (upper_bound as f32 + 2.)
                        );

                        // let primes = primes::primes(n);
                        // println!("Type: {}");
                        // println!("Range: {}");
                        // println!("Prime components: {}");
                }
        }
}

/// I'll be surprised if this works efficiently as a mechanical, literal, procedure.
fn prime_sieve(min: Option<usize>, max: usize) -> Vec<usize> {
        // buncha default yes's
        let mut primes = vec![true; max + 1];
        primes[0] = false;
        primes[1] = false;
        // no need to go past sqrt(n).floor()
        for i in 2..=max.isqrt() {
                // skip if index was marked as multiple of preceding num
                if primes[i] {
                        // first value that's not been sieved would require p >= us, which would be us
                        let mut index = i.pow(2);
                        // false for al p * n indices
                        while index <= max {
                                primes[index] = false;
                                index += i;
                        }
                }
        }
        let min = min.unwrap_or(0);
        // collect unsieved bits
        let mut result = vec![];
        for (i, b) in primes.iter().enumerate().skip(min) {
                if *b {
                        result.push(i);
                }
        }
        result
}
