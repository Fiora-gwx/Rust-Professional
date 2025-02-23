pub fn new_birthday_probability(n: u32) -> f64 {
    if n >= 365 {
        return 1.0; // 超过365天必然存在重复
    }
    
    let mut prob_all_unique = 1.0;
    for i in 0..n {
        prob_all_unique *= (365.0 - i as f64) / 365.0;
    }
    
    let probability = 1.0 - prob_all_unique;
    // 四舍五入到四位小数
    (probability * 10000.0).round() / 10000.0
}