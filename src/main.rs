use clap::Parser;
use std::{cmp::max, time::Instant};

/// Program to find the max subsequence of two strings
/// Based on the following paper: https://ioi.di.unimi.it/maxsubseq.pdf
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The first string to find the LCS & lcs of
    #[arg(long)]
    s1: String,

    /// The second string to find the LCS & lcs of
    #[arg(long)]
    s2: String,

    /// Whether to benchmark the program
    #[arg(short, long)]
    benchmark: bool,

    /// Which algorithm to use, available options are:
    ///
    /// - lcs_for: For Loop
    ///
    /// - lcs_dynamic: Dynamic Programming
    ///
    /// - lcs_rec: Recursive
    #[arg(short, long, default_value = "lcs_for")]
    algorithm: String,
}

fn benchmark<F>(func: F, benchmark: bool) -> Option<std::time::Duration>
where
    F: FnOnce() -> (),
{
    if benchmark {
        let start = Instant::now();
        func();
        Some(start.elapsed())
    } else {
        None
    }
}

fn main() {
    let args = Args::parse();

    println!("s1: {}", args.s1);
    println!("s2: {}", args.s2);

    let duration = benchmark(
        || {
            let lcs = match args.algorithm.as_str() {
                "lcs_for" => lcs_for(&args.s1, &args.s2),
                "lcs_dynamic" => lcs_dynamic(&args.s1, &args.s2),
                "lcs_rec" => lcs_rec(
                    &args.s1,
                    &args.s2,
                    args.s1.len() as i32 - 1,
                    args.s2.len() as i32 - 1,
                ),
                _ => panic!("Invalid algorithm"),
            };

            println!("lcs: {}", lcs);
        },
        args.benchmark,
    );

    if let Some(duration) = duration {
        println!("Time elapsed: {:?}", duration);
    }
}

/// Finds the max subsequence of two strings using a for loop, a vector of already counted characters and
/// the position of the last character that was counted (to make sure that the sequence is valid)
///
/// Kinda the most naive approach, first thing that came to my mind
fn lcs_for(x: &String, y: &String) -> i32 {
    let mut lcs = 0;

    let mut already_counted_chars = vec![];

    // the position of the last character that was counted, if a character in the second word matches, we should make
    // sure that it is after the last character that was counted, otherwise the sequence is not valid per definition
    let mut max_char_pos = 0;

    for i in 0..x.len() {
        let x_char = x.chars().nth(i).unwrap();

        if already_counted_chars.contains(&x_char) {
            continue;
        }

        for j in 0..y.len() {
            let y_char = y.chars().nth(j).unwrap();

            if already_counted_chars.contains(&y_char) {
                continue;
            }

            if x_char == y_char && j > max_char_pos {
                already_counted_chars.push(x_char);
                max_char_pos = j;

                lcs += 1;
            }
        }
    }

    return lcs;
}

/// Finds the max subsequence of two strings using recursion
///
/// The most simple of them all
fn lcs_rec(x: &String, y: &String, i: i32, j: i32) -> i32 {
    if i == -1 || j == -1 {
        return 0;
    }

    if x.chars().nth(i as usize).unwrap() == y.chars().nth(j as usize).unwrap() {
        return 1 + lcs_rec(x, y, i - 1, j - 1);
    } else {
        return max(lcs_rec(x, y, i - 1, j), lcs_rec(x, y, i, j - 1));
    }
}

/// Finds the max subsequence of two strings using dynamic programming and a 2d vector (matrix)
///
/// The most complex of them all
fn lcs_dynamic(x: &String, y: &String) -> i32 {
    let mut dp = vec![vec![0; y.len() + 1]; x.len() + 1];

    for i in 1..=x.len() {
        for j in 1..=y.len() {
            if x.chars().nth(i - 1).unwrap() == y.chars().nth(j - 1).unwrap() {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
            }
        }
    }

    return dp[x.len()][y.len()];
}
