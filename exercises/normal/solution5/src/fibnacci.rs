pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold < 1 {
        return 0; // 阈值小于1时无有效项
    }

    let (mut a, mut b) = (1, 1);
    let mut sum = if threshold >= 1 { 2 } else { 0 }; // 初始两项均为奇数且为1
    
    loop {
        let next = a + b; // 生成下一项
        if next > threshold {
            break;
        }
        if next % 2 != 0 {
            sum += next; // 奇数项累加
        }
        a = b;          // 更新前两项
        b = next;
    }
    
    sum
}
