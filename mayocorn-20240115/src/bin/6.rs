use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    match solve() {
        Some(ans) => println!(
            "{}",
            ans.iter()
                .map(|&(u, v)| format!("{} {}", u + 1, v + 1))
                .join("\n")
        ),
        None => println!("-1"),
    }
}

fn solve() -> Option<Vec<(usize, usize)>> {
    input! {
        (n, m): (usize, usize),
        dd: [usize; n],
        ab: [(Usize1, Usize1); m],
    }

    let mut rem_degrees = dd.clone();
    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        if rem_degrees[a] == 0 || rem_degrees[b] == 0 {
            return None;
        }

        rem_degrees[a] -= 1;
        rem_degrees[b] -= 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    let mut groups = vec![];
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut stack = vec![start];
        let mut group = vec![];
        while let Some(cur) = stack.pop() {
            if visited[cur] {
                continue;
            }

            visited[cur] = true;
            group.push(cur);
            stack.extend(graph[cur].clone());
        }

        groups.push(group);
    }

    let mut single_rem = vec![];
    let mut multi_rem = vec![];
    for group in groups {
        let mut connectable_countries = vec![];
        let mut sum_rem = 0;
        for country in group {
            let rem = rem_degrees[country];

            if rem == 0 {
                continue;
            }

            connectable_countries.push((country, rem));
            sum_rem += rem;
        }

        if sum_rem == 0 {
            return None;
        }

        if sum_rem == 1 {
            single_rem.push(connectable_countries[0].0);
        } else {
            multi_rem.push((connectable_countries, sum_rem));
        }
    }

    let mut roads = vec![];

    while !single_rem.is_empty() && !multi_rem.is_empty() {
        let country1 = single_rem.pop().unwrap();
        let (mut group, mut sum_rem) = multi_rem.pop().unwrap();
        sum_rem -= 1;
        let (country2, mut rem) = group.pop().unwrap();
        rem -= 1;
        if rem != 0 {
            group.push((country2, rem));
        }

        roads.push((country1, country2));

        if sum_rem == 1 {
            single_rem.push(group[0].0);
        } else {
            multi_rem.push((group, sum_rem));
        }
    }

    if single_rem.len() != 2 || !multi_rem.is_empty() {
        return None;
    }

    roads.push((single_rem[0], single_rem[1]));

    Some(roads)
}
