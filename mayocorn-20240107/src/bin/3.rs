use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        s: String,
    }

    let char_to_usize = |c: char| (c as u8 - b'a') as usize;

    let mut contains = vec![false; 26];
    let mut stack = vec![];
    for c in s.chars() {
        if c == ')' {
            while let Some(cur) = stack.pop() {
                if cur == '(' {
                    break;
                }

                contains[char_to_usize(cur)] = false;
            }
        } else if c == '(' {
            stack.push(c);
        } else {
            stack.push(c);

            let idx = char_to_usize(c);

            if contains[idx] {
                return false;
            }

            contains[idx] = true;
        }
    }

    true
}
