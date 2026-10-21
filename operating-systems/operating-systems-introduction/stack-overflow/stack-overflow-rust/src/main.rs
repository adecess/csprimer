fn main() {
    start();
    println!("OK");
}

fn start() {
    let depth = 0i32;
    f(depth, &depth);
}

#[inline(never)]
fn f(depth: i32, bottom: &i32) {
    let bottom_addr = bottom as *const i32 as isize;
    let current_addr = &depth as *const i32 as isize;

    println!(
        "frame #{} {} ({:p})",
        depth,
        bottom_addr - current_addr,
        &depth
    );

    f(depth + 1, bottom);
}
