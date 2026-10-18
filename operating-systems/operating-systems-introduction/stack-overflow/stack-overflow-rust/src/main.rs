use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1]
        .parse::<u32>()
        .expect("Failed to parse the argument into a u32");

    let mut frames = 0u32;
    println!("fib({:?}) is {}", n, boom(n, &mut frames));
}

fn boom(n: u32, frames: &mut u32) -> u32 {
    println!("stack frame #{}", frames);
    *frames += 1;
    println!("current stack address of n {:p}", &n);

    let boom_number = boom(n + 1, frames);

    *frames -= 1;
    boom_number
}
