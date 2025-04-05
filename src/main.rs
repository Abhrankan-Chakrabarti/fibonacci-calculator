use anyhow::Result;
use clap::Parser;
use malachite::Integer;
use malachite::num::arithmetic::traits::Square;
use malachite::num::basic::traits::{One, Two, Zero};
use std::{io::{self, Write}, time::Instant};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    help_template = "\
{name} {version}
{author}
{about}

USAGE:
    {usage}

OPTIONS:
{options}
"
)]
struct Args {
    /// Suppress printing the Fibonacci result
    #[arg(long)]
    no_print: bool,
}

fn input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;
    Ok(n.trim().to_string())
}

fn parse_input(s: &str) -> Option<i32> {
    if let Some(stripped) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i32::from_str_radix(stripped, 16).ok()
    } else if s.contains('e') || s.contains('E') {
        s.parse::<f64>().ok().map(|f| f.round() as i32)
    } else {
        s.parse().ok()
    }
}

fn fib(n: i32) -> Integer {
    if n == 0 {
        return Integer::ZERO;
    }
    if n == 1 {
        return Integer::ONE;
    }
    if n % 2 == 0 {
        let a = fib(n / 2);
        let b = fib(n / 2 - 1);
        return &a * (Integer::TWO * &b + &a);
    }
    let a = fib((n + 1) / 2);
    let b = fib((n - 1) / 2);
    return a.square() + b.square();
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("This program computes the n-th Fibonacci number.");
    loop {
        let raw = input("Enter n (or negative to quit): ")?;
        let n: i32 = match parse_input(&raw) {
            Some(n) => n,
            None => {
                eprintln!("Invalid input. Please enter a valid number.");
                continue;
            }
        };
        if n < 0 {
            break;
        }

        let start = Instant::now();
        let result = fib(n);
        let elapsed = start.elapsed();

        if !args.no_print {
            println!("F({})\t= {}", n, result);
        }
        println!("Elapsed time:\t{:.6} sec", elapsed.as_secs_f64());
    }

    Ok(())
}
