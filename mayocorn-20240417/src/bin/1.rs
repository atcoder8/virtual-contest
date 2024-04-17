use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        image: [String; h],
    }

    println!("{}", "#".repeat(w + 2));
    for line in image {
        println!("#{}#", line);
    }
    println!("{}", "#".repeat(w + 2));
}
