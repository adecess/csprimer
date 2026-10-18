use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1]
        .parse::<u64>()
        .expect("Failed to parse the argument into a u64");

    let mut frames = 0u64;
    println!("fib({:?}) is {}", n, boom(n, &mut frames));
}

fn boom(n: u64, frames: &mut u64) -> u64 {
    println!("stack frame #{}", frames);
    *frames += 1;
    println!("current stack address of n {:p}", &n);

    let boom_number = boom(n + 1, frames);

    *frames -= 1;
    boom_number
}