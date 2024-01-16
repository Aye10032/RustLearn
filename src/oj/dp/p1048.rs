use std::io::stdin;

pub fn p1048() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().split(" ");
    let total_time = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let mut velue: Vec<usize> = Vec::new();
    let mut time: Vec<usize> = Vec::new();
    let mut dp: Vec<i64> = vec![0; total_time + 1];

    velue.push(0);
    time.push(0);

    for _ in 1..=m {
        input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split(" ");
        let t = iter.next().unwrap().parse::<usize>().unwrap();
        let v = iter.next().unwrap().parse::<usize>().unwrap();

        time.push(t);
        velue.push(v);
    }

    // println!("{:?}", time);
    // println!("{:?}", velue);

    for i in 1..=m {
        for j in (time[i]..=total_time).rev() {
            dp[j] = dp[j].max(dp[j - time[i]] + velue[i] as i64);
        }
    }

    let result = dp[total_time];
    println!("{result}");
}