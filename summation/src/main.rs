

fn main() {
    println!("{}", sum_formula(1_000_000_000));
    println!("{}", sum_loop(1_000_000_000));
}

fn sum_formula(n: u128) -> u128 {
    let sum = (n / 2) * (n - 1);

    sum
}

fn sum_loop(n: u128) -> u128 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }

    sum
}
