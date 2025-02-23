pub fn goldbach_conjecture() -> String {
    let mut results = vec![];
    let mut n = 9;
    while results.len() < 2 {
        if !is_prime(n) {
            let max_k = ((n - 2) as f64 / 2.0).sqrt() as u64;
            let mut found = false;
            for k in 1..=max_k {
                let p = n - 2 * k * k;
                if is_prime(p) {
                    found = true;
                    break;
                }
            }
            if !found {
                results.push(n);
            }
        }
        n += 2;
    }
    format!("{},{}", results[0], results[1])
}
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    let mut w = 2;
    let max = (n as f64).sqrt() as u64;
    while i <= max {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }
    true
}
