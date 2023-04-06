fn say_hello() {
    println!("Hello!!!");
}

fn add_numbers(x: usize, y: usize) -> usize {
    x + y
}

fn main() {
    say_hello();

    let mut x = 5 * 6;
    println!("value of x is {x}");
    x = 6;
    println!("value of x now is {x}");

    const HOURS_IN_SECONDS: u32 = 60 * 60;
    println!("value of hours_in_seconds now is {HOURS_IN_SECONDS}");

    let y = 20;
    let y = y + 12;

    println!("y: {y}");

    let sum_ab = add_numbers(x, y);

    println!("sum of a & b is {sum_ab}")
}
