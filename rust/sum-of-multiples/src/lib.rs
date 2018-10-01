pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;

    for x in 1..limit {
         let x_divisible_by_any = factors
            .clone()
            .into_iter()
            .map(|y| x >= *y && x % *y == 0) // x is divisible by y
            .filter(|b| *b)
            .collect::<Vec<bool>>()
            .len() > 0;

         if x_divisible_by_any {
             sum += x;
         }
    }
    return sum;
}
