use std::io::Write;

fn main() {
    let (n, _m) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut visited = vec![false; n];
    visited[0] = true;

    rec(0, &mut visited);
}

fn rec(cur: usize, visited: &mut [bool]) -> bool {
    let judge = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<String>().unwrap()
    };

    assert_ne!(judge, "-1");

    if judge == "OK" {
        return true;
    }

    let vv: Vec<_> = judge
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect();

    for &next in &vv {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        println!("{}", next + 1);
        std::io::stdout().flush().unwrap();

        if rec(next, visited) {
            return true;
        }

        println!("{}", cur + 1);
        std::io::stdout().flush().unwrap();

        let mut _line = String::new();
        std::io::stdin().read_line(&mut _line).unwrap();
    }

    false
}
