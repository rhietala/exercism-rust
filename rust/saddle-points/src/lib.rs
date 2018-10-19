pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.len() == 0 || input[0].len() == 0 {
        return vec![];
    }

    let mut ret: Vec<(usize, usize)> = vec![];

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let row: Vec<u64> = input[y].clone();
            let col: Vec<u64> = get_column(input, x);

            if compare(row, |a| { input[y][x] >= a }) &&
                compare(col, |a| { input[y][x] <= a }) {
                ret.push((y, x));
            }
        }
    }

    ret
}

pub fn get_column(input: &[Vec<u64>], x: usize) -> Vec<u64> {
    input.clone().iter().map(|row| row[x]).collect()
}

pub fn compare<F: Fn(u64) -> bool>(row: Vec<u64>, comparator: F) -> bool {
    row.clone()
        .into_iter()
        .map(comparator)
        .filter(|b| *b)
        .collect::<Vec<bool>>()
        .len() == row.len()
}
