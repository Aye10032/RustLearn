use std::cmp::max;
use std::io::stdin;

pub fn p1802() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().split(" ");
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let x = iter.next().unwrap().parse::<usize>().unwrap();

    let mut win: Vec<usize> = vec![0; n + 1];
    let mut loss: Vec<usize> = vec![0; n + 1];
    let mut use_m: Vec<usize> = vec![0; n + 1];
    for i in 1..=n {
        input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split(" ");
        loss[i] = iter.next().unwrap().parse::<usize>().unwrap();
        win[i] = iter.next().unwrap().parse::<usize>().unwrap();
        use_m[i] = iter.next().unwrap().parse::<usize>().unwrap();
    }

    let mut dp: Vec<usize> = vec![0; x + 1];
    for i in 1..=n {
        for j in (0..=x).rev() {
            if j >= use_m[i] {
                dp[j] = max(dp[j] + loss[i], dp[j - use_m[i]] + win[i])
            } else {
                dp[j] = dp[j] + loss[i];
            }
        }
    }

    let result = 5 * dp[x];
    println!("{result}")
}