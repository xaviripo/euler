fn main() {
    let mut palindromes: Vec<u64> = vec![];
    for n in 1..999 {
        for m in 1..999 {
            let candidate = n * m;
            let text = candidate.to_string();
            let mid = text.len()/2;
            if &text[..if mid % 2 == 0 {mid+1} else {mid}] == &text[mid..].chars().rev().collect::<String>() {
                palindromes.push(candidate);
            }
        }
    }
    println!("{:?}", palindromes.iter().max().unwrap());
}
