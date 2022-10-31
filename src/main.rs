fn main() {
    let fib = calculate_fibonacci(23);
    println!("{}", fib);
}

fn calculate_fibonacci(n: i32) -> i32 {
    let mut fib1 = 1;
    let mut fib2 = 1;

    let mut i = 0;

    while i < n - 2 {
        let fib_sum = fib1 + fib2;
        fib1 = fib2;
        fib2 = fib_sum;
        i += 1;
    };
    fib2
}