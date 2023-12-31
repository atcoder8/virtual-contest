use proconio::input;

fn main() {
    input! {
        (a, b): (char, char),
    }

    let honest = (a == 'H') == (b == 'H');
    println!("{}", if honest { 'H' } else { 'D' });
}
