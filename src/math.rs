pub fn lcm(values: &[u64]) -> u64 {
    let mut result = values[0];
    for &v in &values[1..] {
        result = result * v / gcd(result, v);
    }
    result
}

pub fn gcd(p0: u64, p1: u64) -> u64 {
    let mut a = p0;
    let mut b = p1;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

