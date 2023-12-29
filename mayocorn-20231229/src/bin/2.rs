use im_rc::HashSet;
use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("satisfiable"),
    }
}

fn solve() -> Option<String> {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut added = HashSet::new();
    for s in &ss {
        if s.chars().next().unwrap() == '!' {
            added.insert(s[1..].to_owned());
        }
    }

    for s in ss {
        if s.chars().next().unwrap() != '!' {
            if added.contains(&s) {
                return Some(s);
            }
        }
    }

    None
}
