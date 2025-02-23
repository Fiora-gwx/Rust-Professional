pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // Step 1: Parse the input string (e.g., "10(2)" -> number "10", base 2)
    let (number, from_base) = parse_num_str(num_str);

    // Step 2: Convert the number from source base to decimal
    let decimal = base_to_decimal(&number, from_base);

    // Step 3: Convert decimal to target base
    decimal_to_base(decimal, to_base)
}

// Parse the input string into number and source base
fn parse_num_str(num_str: &str) -> (String, u32) {
    let parts: Vec<&str> = num_str.split('(').collect();
    let number = parts[0].to_string();
    let base_str = parts[1].trim_end_matches(')');
    let base = base_str.parse::<u32>().expect("Invalid base in input");
    (number, base)
}

// Convert a number from given base to decimal
fn base_to_decimal(num: &str, from_base: u32) -> u64 {
    let mut result = 0;
    for (i, c) in num.chars().rev().enumerate() {
        let digit = char_to_value(c);
        result += digit as u64 * (from_base as u64).pow(i as u32);
    }
    result
}

// Convert a decimal number to the target base
fn decimal_to_base(mut num: u64, to_base: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut digits = Vec::new();
    while num > 0 {
        let remainder = (num % to_base as u64) as u32;
        digits.push(value_to_char(remainder));
        num /= to_base as u64;
    }

    digits.into_iter().rev().collect()
}

// Convert a character to its numeric value (0-9, a-z)
fn char_to_value(c: char) -> u32 {
    match c {
        '0'..='9' => c as u32 - '0' as u32,
        'a'..='z' => c as u32 - 'a' as u32 + 10,
        'A'..='Z' => c as u32 - 'A' as u32 + 10,
        _ => panic!("Invalid digit: {}", c),
    }
}

// Convert a numeric value to its character representation
fn value_to_char(value: u32) -> char {
    if value < 10 {
        if let Some(c) = char::from_u32(value + u32::from(b'0')) {
            c
        } else {
            // 处理无效 Unicode 字符的情况，例如返回默认字符
            '?'
        }
    } else {
        // 添加安全转换
        if let Some(c) = char::from_u32(value - 10 + u32::from(b'a')) {
            c
        } else {
            // 处理无效 Unicode 字符的情况，例如返回默认字符
            '?'
        }
    }
}