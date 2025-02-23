pub fn dp_rec_mc(amount: u32) -> u32 {
    // 如果金额为0,返回0
    if amount == 0 {
        return 0;
    }
    
    // 可用的纸币面额
    let coins = vec![1, 2, 5, 10, 20, 30, 50, 100];
    
    // 创建dp数组,初始化为最大值
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;
    
    // 对每个金额进行动态规划
    for i in 1..=amount {
        // 尝试每种面额的纸币
        for &coin in coins.iter() {
            if coin <= i {
                // 如果可以使用当前面额
                let prev = dp[(i - coin) as usize];
                if prev != u32::MAX {
                    // 更新最小纸币数
                    dp[i as usize] = dp[i as usize].min(prev + 1);
                }
            }
        }
    }
    
    // 返回结果
    dp[amount as usize]
}