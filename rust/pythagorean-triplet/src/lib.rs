pub fn find() -> Option<u32> {
    for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if rule1(a, b, c) && rule2(a, b, c) {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}

pub fn rule1(a: u32, b: u32, c: u32) -> bool {
    a*a + b*b == c*c
}

pub fn rule2(a: u32, b: u32, c: u32) -> bool {
    a + b + c == 1000
}
