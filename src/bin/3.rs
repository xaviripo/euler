fn main() {
    let mut target: u64 = 600851475143;
    let max: u64 = (target as f64).sqrt().ceil() as u64;
    let mut divisors: Vec<u64> = vec![];
    for n in 2..max {
        if target % n == 0 {
            divisors.push(n);
            while target % n == 0 {
                target = target / n;
            }
        }
    }
    println!("{:?}", divisors.iter().max().unwrap());
}
