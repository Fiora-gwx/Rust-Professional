use rand::Rng;

// 计算最大公约数 (GCD)
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Pollard's Rho 算法分解大整数
fn pollard_rho(n: u128) -> Option<u128> {
    if n < 2 {
        return None;
    }
    if n % 2 == 0 {
        return Some(2);
    }

    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(2..n.min(1_000_000)) % n;
    let mut y = x;
    let c = rng.gen_range(1..n.min(1_000_000)) % n;
    let mut d = 1;

    while d == 1 {
        x = (x.wrapping_mul(x) % n).wrapping_add(c) % n;
        y = (y.wrapping_mul(y) % n).wrapping_add(c) % n;
        y = (y.wrapping_mul(y) % n).wrapping_add(c) % n;
        d = gcd(if x > y { x - y } else { y - x }, n);
        if d != n && d != 1 {
            return Some(d);
        }
    }
    None
}

// Miller-Rabin 素性测试
fn miller_rabin(n: u128, k: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut s = 0;
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();
    'witness: for _ in 0..k {
        let a = rng.gen_range(2..n.min(1_000_000)) % (n - 2) + 2;
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

// 模乘幂运算
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result as u128).wrapping_mul(base) % modulus;
        }
        base = (base as u128).wrapping_mul(base) % modulus;
        exp /= 2;
    }
    result
}

// 主函数：找出最大素数因子
pub fn find_max_prime_factor(mut n: u128) -> u128 {
    let mut factors = Vec::new();

    // 预处理小素数因子
    for p in 2..1000 {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }

    // 处理剩余大因子
    while n > 1 {
        if miller_rabin(n, 20) {
            factors.push(n);
            break;
        }
        if let Some(factor) = pollard_rho(n) {
            factors.push(find_max_prime_factor(factor));
            n /= factor;
        } else {
            factors.push(n);
            break;
        }
    }

    // 返回最大素数因子
    *factors.iter().max().unwrap()
}