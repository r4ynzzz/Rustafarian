use std::collections::HashMap;

fn median(numbers: &mut [i32]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    numbers.sort_unstable();

    let len = numbers.len();
    let mid = len / 2;

    if len % 2 == 0 {
        Some((numbers[mid - 1] + numbers[mid]) as f64 / 2.0)
    } else {
        Some(numbers[mid] as f64)
    }
}

fn mode(numbers: &[i32]) -> Vec<i32> {
    let mut counts = HashMap::new();

    //1. Count occurrences
    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    //2. Find the highest frequency
    let max_count = counts.values().cloned().max().unwrap_or(0);

    //Collect all values that match highest frequency
    let modes: Vec<i32> = counts
        .into_iter()
        .filter(|&(_, count)| count == max_count)
        .map(|(num, _)| num)
        .collect();

    modes
}

fn main() {
    let mut numbers = vec![19, 12, 43, 64, 25, 16, 97, 58, 22, 10, 43];

    println!("Mode: {:?}", mode(&numbers));
    println!("Median: {:?}", median(&mut numbers));
}
