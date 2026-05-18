fn main() {
    for value in 0..15 {
        println!("Fibonacci ({}) => {}", value, fib(value));
    }
}

fn fib(n:i32) -> i64 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}