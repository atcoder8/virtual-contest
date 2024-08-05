use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, c): (usize, usize),
        mut abc: [(Usize1, usize, usize); n],
    }

    let events = abc
        .iter()
        .flat_map(|&(a, b, c)| create_event_pair(a, b, c))
        .sorted_unstable_by_key(|event| event.day);

    let mut total_cost = 0;
    let mut prev_day = 0;
    let mut cost_per_day = 0;
    for event in events {
        total_cost += cost_per_day.min(c) * (event.day - prev_day);
        if event.join {
            cost_per_day += event.cost;
        } else {
            cost_per_day -= event.cost;
        }
        prev_day = event.day;
    }

    println!("{}", total_cost);
}

struct Event {
    day: usize,
    cost: usize,
    join: bool,
}

fn create_event_pair(a: usize, b: usize, c: usize) -> [Event; 2] {
    [
        Event {
            day: a,
            cost: c,
            join: true,
        },
        Event {
            day: b,
            cost: c,
            join: false,
        },
    ]
}
