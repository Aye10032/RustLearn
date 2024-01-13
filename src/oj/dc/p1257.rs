use std::io::stdin;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}


pub fn p1257() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut points: Vec<Point> = Vec::new();
    for _ in 0..n {
        input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split(" ");
        let p = Point {
            x: iter.next().unwrap().parse::<i64>().unwrap(),
            y: iter.next().unwrap().parse::<i64>().unwrap(),
        };
        points.push(p);
    }

    sort_by_x(&mut points);

    let min_dis = find_min(&mut points, 0, n - 1);

    println!("{:.4}", min_dis);
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p1.x - p2.x) as f64;
    let dy = (p1.y - p2.y) as f64;
    return (dx.powi(2) + dy.powi(2)).sqrt();
}

fn sort_by_x(points: &mut Vec<Point>) {
    points.sort_by(|a, b| a.x.cmp(&b.x));
}

fn sort_by_y(points: &mut Vec<Point>) {
    points.sort_by(|a, b| a.y.cmp(&b.y));
}

fn find_min(points: &mut Vec<Point>, l: usize, r: usize) -> f64 {
    if l == r {
        return f64::MAX;
    }

    if r - l == 1 {
        return distance(&points[l], &points[r]);
    }

    let pivot = (l + r) >> 1;
    let min_l = find_min(points, l, pivot);
    let min_r = find_min(points, pivot + 1, r);
    let mut min_part = min_l.min(min_r);

    let mid_x = (points[pivot].x + points[pivot + 1].x) >> 1;

    let mut sigma: Vec<Point> = Vec::new();
    for i in l..=r {
        if (points[i].x - mid_x).abs() as f64 <= min_part {
            sigma.push(points[i].clone());
        }
    }

    sort_by_y(&mut sigma);

    for i in 0..sigma.len() {
        for j in i + 1..sigma.len() {
            if (sigma[j].y - sigma[i].y).abs() as f64 > min_part {
                break;
            }
            min_part = min_part.min(distance(&sigma[i], &sigma[j]));
        }
    }

    return min_part;
}