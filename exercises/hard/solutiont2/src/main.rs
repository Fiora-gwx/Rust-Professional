// I AM NOT DONE

mod prime_factor;
use std::time::{Instant, Duration};
fn main() {
    let start = Instant::now();
            
    let number = 199999999999999951437;
    let duration = start.elapsed();
    let res = prime_factor::find_max_prime_factor(number);
    println!("Time: {:?}", duration);
    println!("{number}'s max prime factor: {res}");
}
