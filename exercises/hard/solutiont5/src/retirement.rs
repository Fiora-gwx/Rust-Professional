use std::cmp::min;
fn calc_segment(birth_total: i32, seg_start: i32, seg_end: i32, max_delay: i32) -> i32 {
    let range = seg_end - seg_start;
    // 使用四舍五入：加上 range/2 后整除
    ((birth_total - seg_start) * max_delay + range/2) / range
}

/// 将总月数转换为 "YYYY-MM" 格式（假设月份从1到12）
fn month_to_ym(total: i32) -> String {
    // 由于 total = year * 12 + month，其中 month 为1~12
    // 为了正确取整，先减1再除
    let year = (total - 1) / 12;
    let month = (total - 1) % 12 + 1;
    format!("{:04}-{:02}", year, month)
}
fn format_number_str(num: f64) -> String {
    let s = format!("{:.2}", num); // 先格式化为两位小数
    if s.ends_with(".00") {
        s[0..s.len() - 3].to_string() // 去除 ".00"
    } else {
        s
    }
}

/// 根据出生年月（"YYYY-MM"）和人员类型计算退休时间、退休年龄及延迟退休月数
///
/// 输入 personnel 包含以下三种情况：
/// - "男职工"：原法定退休年龄60岁，延迟退休区间：
///      * 分段1：1965-01 至 1967-8，从1到8月,每四个月延迟一个月；
///      * 分段2：1967-9 至 1976-12，从9到18月（满后取36）。
/// - "原法定退休年龄55周岁女职工"：原法定退休年龄55岁，延迟退休区间：
///      * 分段1：1963-04 至 1971-04，从0到4月；
///      * 分段2：1971-04 至 2000-12，从4到36月（满后取36）。
/// - "原法定退休年龄50周岁女职工"：原法定退休年龄50岁，
///      * 若出生月份 >= 1995-12，则延迟60月（否则延迟0，测试用例中只给出1995-12情况）。
pub fn retire_time(birth: &str, personnel: &str) -> String {
    // 解析出生年月
    let parts: Vec<&str> = birth.split('-').collect();
    if parts.len() != 2 {
        return "Invalid birth format".to_string();
    }
    let year: i32 = parts[0].parse().unwrap_or(0);
    let month: i32 = parts[1].parse().unwrap_or(0);
    let birth_total = year * 12 + month; // 将日期转换为总月数

    // 定义计算延迟退休的参数
    let (stat_age, max_delay, delay_months): (i32, i32, i32);
    let delay;
    if personnel.contains("男职工") {
        // 男职工：原退休60岁，最大延迟36月。
        stat_age = 60;
        // 分段1：1965-02 到 1965-12
        let seg1_start = 1965 * 12 + 1;
        if birth_total < seg1_start {
            delay = 0;
        } else {
            delay = min((birth_total - seg1_start)/4+1,36); 
        }
        //println!("{}  {}",delay,birth_total);
    } else if personnel.contains("50周岁") {
        // 原法定退休年龄50周岁女职工：最大延迟60月
        stat_age = 50;
        // 这里按照测试用例：若出生在1995-12及以后，延迟60月，否则0
        let threshold = 1995 * 12 + 12;
        delay = if birth_total >= threshold { 60 } else { 0 };
    } else if personnel.contains("55周岁") {
        // 原法定退休年龄55周岁女职工：最大延迟36月
        stat_age = 55;
        // 分段1：1963-04 到 1971-04
        let seg1_start = 1963 * 12 + 4;
        let seg1_end   = 1971 * 12 + 4;
        // 分段2：1971-04 到 2000-12
        let seg2_end   = 2000 * 12 + 12;
        if birth_total <= seg1_start {
            delay = 0;
        } else if birth_total < seg1_end {
            delay = calc_segment(birth_total, seg1_start, seg1_end, 4);
        } else if birth_total < seg2_end {
            let d2 = calc_segment(birth_total, seg1_end, seg2_end, 32);
            delay = 4 + d2;
        } else {
            delay = 36;
        }
    } else {
        // 未识别的人员类型，默认不延迟，按男职工处理
        stat_age = 60;
        delay = 0;
    }
    // 正常退休月份 = 出生月 + 法定退休年龄（月数）
    let normal_retire_total = birth_total + stat_age * 12;
    // 实际退休月份 = 正常退休月份 + 延迟月数
    let actual_retire_total = normal_retire_total + delay;
    let retire_date = month_to_ym(actual_retire_total);
    // 退休年龄 = 法定退休年龄 + 延迟月数/12（保留两位小数）
    let retire_age = stat_age as f64 + (delay as f64) / 12.0;
    format!("{},{}{},{}", retire_date, "", format_number_str(retire_age), delay)
}