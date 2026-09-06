use nix::sched::sched_getcpu; // nix provides safe bindings to libc
use std::hint::black_box;
use std::time::Instant;
use std::{process, thread, time};

const SLEEP_SEC: time::Duration = time::Duration::from_millis(3000);
const NUM_MULS: u32 = 100000000;
const NUM_ALLOCS: u32 = 100000;
const ALLOC_SIZE: usize = 1000;

#[derive(Default)]
struct ProfileTimes {
    start: Option<Instant>,
    elapsed: f32,
    user_time: f32,
    sys_time: f32
}

fn profile_start(profile_times: &mut ProfileTimes, log: &str) {
    println!("[pid {}, cpu {}] {}", process::id(), sched_getcpu().unwrap(), log);

    profile_times.start = Some(Instant::now());
}

fn profile_log(profile_times: &mut ProfileTimes) {
    profile_times.elapsed = profile_times.start.unwrap().elapsed().as_secs_f32();

    println!(
        "[pid {}, cpu {}] real {:?}",
        process::id(),
        sched_getcpu().unwrap(),
        profile_times.elapsed
    )
}

fn main() {
    let mut profile_times = ProfileTimes::default();

    let mut x = 1.0f32;
    let mut i = 0_32;
    profile_start(&mut profile_times, &format!("{} fmuls", NUM_MULS));
    while i < NUM_MULS {
        // avoid Rust-specific iterator overhead
        x *= 1.1f32;
        i += 1;
    }
    black_box(&x);
    profile_log(&mut profile_times);

    profile_start(
        &mut profile_times,
        &format!("{} allocs of size {}", NUM_ALLOCS, ALLOC_SIZE),
    );
    for _ in 0..NUM_ALLOCS {
        let ptr = vec![0; ALLOC_SIZE]; // simple and safe way to heap allocate a specific size
        black_box(ptr);
    }
    profile_log(&mut profile_times);

    profile_start(&mut profile_times, &format!("sleeping for {:?}", SLEEP_SEC));
    thread::sleep(SLEEP_SEC);
    profile_log(&mut profile_times);
}
