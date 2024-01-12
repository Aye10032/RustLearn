use std::io::stdin;

pub fn p1177() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut a: Vec<i32> = input.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).collect();

    sort(&mut a, 0, n - 1);

    let sorted_a = a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{sorted_a}");
}

fn sort(a: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let pivot = start + (end - start) / 2;

    sort(a, start, pivot);
    sort(a, pivot + 1, end);
    merge(a, start, end, pivot);
}

fn merge(a: &mut Vec<i32>, start: usize, end: usize, pivot: usize) {
    let mut temp: Vec<i32> = Vec::new();
    let mut left = start;
    let mut right = pivot + 1;


    while left <= pivot && right <= end {
        if a[left] <= a[right] {
            temp.push(a[left]);
            left += 1;
        } else {
            temp.push(a[right]);
            right += 1;
        }
    }

    while left <= pivot {
        temp.push(a[left]);
        left += 1;
    }

    while right <= end {
        temp.push(a[right]);
        right += 1;
    }

    for i in start..=end {
        a[i] = temp[i - start];
    }
}