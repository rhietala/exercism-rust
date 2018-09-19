pub fn find() -> Option<u32> {
    for a in 1..1000 {
        for b in 1..a {
            // a + b + c == 1000, therefore
            let c = 1000 - a - b;

            // sum_equals_1000 check should always hold
            if is_pythagorean_triplet(a, b, c) && sum_equals_1000(a, b, c) {
                return Some(a * b * c);
            }
        }
    }
    None
}

pub fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    a*a + b*b == c*c
}

pub fn sum_equals_1000(a: u32, b: u32, c: u32) -> bool {
    a + b + c == 1000
}
