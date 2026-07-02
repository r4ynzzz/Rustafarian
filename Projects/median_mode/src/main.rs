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

fn mode(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut map = HashMap::new();

    for n in numbers {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }
}

fn main() {
    let mut numbers = vec![19, 12, 43, 64, 25, 16, 97, 58, 22, 10, 43];

    println!("Median: {:?}", median(&mut numbers))
}
