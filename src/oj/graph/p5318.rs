use std::io::stdin;
use std::collections::VecDeque;

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
}

impl Edge {
    fn new(u: usize, v: usize) -> Edge {
        Edge { u, v }
    }
}

pub fn p5318() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split(" ");
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let visit: Vec<bool> = vec![false; n + 1];
    let mut edge_list: Vec<Edge> = vec![Edge::new(0, 0)];
    let mut graph: Vec<Vec<usize>> = vec![vec![0]; n + 1];

    for _ in 0..m {
        input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split(" ");
        let x = iter.next().unwrap().parse::<usize>().unwrap();
        let y = iter.next().unwrap().parse::<usize>().unwrap();

        edge_list.push(Edge::new(x, y))
    }

    edge_list.sort_unstable_by(|a, b| {
        if a.v != b.v {
            a.v.cmp(&b.v)
        } else {
            a.u.cmp(&b.u)
        }
    });

    // for i in 1..=m {
    //     println!("{:?}", edge_list[i]);
    // }

    for i in 1..edge_list.len() {
        graph[edge_list[i].u].push(edge_list[i].v);
    }

    // for i in 1..=n {
    //     println!("{:?}", graph[i]);
    // }

    let mut v1: Vec<bool> = visit.clone();
    dfs(&graph, &mut v1, 1);
    println!();

    let mut v2: Vec<bool> = visit.clone();
    bfs(&graph, &mut v2, 1);
}

fn dfs(graph: &Vec<Vec<usize>>,
       visit: &mut Vec<bool>,
       v: usize,
) {
    visit[v] = true;
    print!("{} ", v);

    for i in 1..graph[v].len() {
        if !visit[graph[v][i]] {
            dfs(graph, visit, graph[v][i]);
        }
    }
}

fn bfs(graph: &Vec<Vec<usize>>,
       visit: &mut Vec<bool>,
       v: usize,
) {
    let mut queue = VecDeque::new();

    visit[v] = true;
    queue.push_back(v);

    while let Some(_v) = queue.pop_front() {
        print!("{} ", _v);

        for i in 1..graph[_v].len() {
            if !visit[graph[_v][i]] {
                visit[graph[_v][i]] = true;
                queue.push_back(graph[_v][i]);
            }
        }
    }
}