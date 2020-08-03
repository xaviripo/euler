fn main() {
    let mut pair = (0, 1);
    let mut total = 0;
    while pair.0 <= 4_000_000 {
        total += pair.0;

        // Make three iterations at once because the pattern is even, odd, odd, even, odd, odd, ...
        pair = next_even(pair);
    }
    println!("{}", total);
}

fn next_fib((a, b): (u32, u32)) -> (u32, u32) {
    (b, a + b)
}

fn next_even(pair: (u32, u32)) -> (u32, u32) {
    next_fib(next_fib(next_fib(pair)))
}
