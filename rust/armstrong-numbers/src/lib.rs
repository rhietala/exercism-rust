pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = digits_for_number(num);
    let digits_len: u32 = digits.len() as u32;
    let armstrong_number: u32 = digits
        .into_iter()
        .map(|x| x.pow(digits_len))
        .sum();
    return num == armstrong_number;
}

pub fn digits_for_number(num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    let mut num: u32 = num;

    if num == 0 {
        digits.push(num);
    }

    // This algorithm wouldn't work for negative numbers or floating-point
    // numbers. Luckily num is u32.
    while num >= 1 {
        digits.push(num % 10);
        num /= 10;
    }

    digits.reverse();
    return digits;
}
