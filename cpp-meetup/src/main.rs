fn main() {
    let mut cache: Vec<u64> = Vec::new();
    for p in 2u64.. {
        let has_divisor = cache
            .iter()
            .take_while(|&c| c * c <= p)
            .any(|c| p % c == 0);
        if !has_divisor {
            println!("{}", p);
            cache.push(p);
        }
    }
}
