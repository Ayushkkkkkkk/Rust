use std::io::{self, BufRead};

fn count_numbers(a: u64, b: u64) -> u64 {
    let a_digits: Vec<u8> = a
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let b_digits: Vec<u8> = b
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut dp = vec![vec![0; 10]; 20];
    dp[0][0] = 1;

    for i in 1..20 {
        for d in 0..10 {
            for last_digit in 0..10 {
                if d != last_digit {
                    dp[i][d] += dp[i - 1][last_digit];
                }
            }
        }
    }

    let count_a = count_valid_numbers(&dp, &a_digits);
    let count_b = count_valid_numbers(&dp, &b_digits);

    count_b - count_a
}

fn count_valid_numbers(dp: &Vec<Vec<u64>>, digits: &Vec<u8>) -> u64 {
    let mut count = 0;
    let len = digits.len();

    for i in 1..len {
        for d in 1..10 {
            count += dp[i][d];
        }
    }

    for i in 0..digits[0] {
        count += dp[len - 1][i as usize];
    }

    for i in 1..len {
        for d in 0..digits[i] {
            if digits[i - 1] != d {
                count += dp[len - i - 1][d as usize];
            }
        }
        if digits[i] == digits[i - 1] {
            break;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut split = line.split_whitespace();

    let a: u64 = split.next().unwrap().parse().unwrap();
    let b: u64 = split.next().unwrap().parse().unwrap();

    let result = count_numbers(a, b);
    println!("{}", result);
}

