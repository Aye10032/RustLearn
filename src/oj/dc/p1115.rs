use std::cmp::max;
use std::io::stdin;

const MIN: i32 = -i32::MAX;
pub fn p1115() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut a: Vec<i32> = input.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

    let max = rec(&mut a, 0, n - 1);
    println!("{max}");
}

fn rec(a: &mut Vec<i32>, l: usize, r: usize) -> i32 {
    if l == r {
        return a[l];
    }

    let pivot = (l + r) >> 1;
    let mut max_l = MIN;
    let mut max_r = MIN;

    let mut sum = 0;
    for i in (l..=pivot).rev() {
        sum += a[i];
        max_l = max(max_l, sum);
    }

    sum = 0;
    for i in pivot + 1..=r {
        sum += a[i];
        max_r = max(max_r, sum);
    }

    return max(max_l + max_r, max(rec(a, l, pivot), rec(a, pivot + 1, r)));
}