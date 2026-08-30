use rustix::thread::sched_getcpu; // rustix provides safe bindings to POSIX APIs
use std::alloc::{Layout, alloc};
use std::hint::black_box;
use std::time::Instant;
use std::{process, thread, time};

const SLEEP_SEC: time::Duration = time::Duration::from_millis(3000);
const NUM_MULS: u32 = 100000000;
const NUM_ALLOCS: u32 = 100000;
const ALLOC_SIZE: usize = 1000;
const ALIGN_SIZE: usize = 16; // default malloc align size on x86-64 Unix systems

#[derive(Default)]
struct ProfileTimes {
    start: Option<Instant>,
    elapsed: f32,
}

fn profile_start(profile_times: &mut ProfileTimes, log: &str) {
    println!("[pid {}, cpu {}] {}", process::id(), sched_getcpu(), log);

    profile_times.start = Some(Instant::now());
}

fn profile_log(profile_times: &mut ProfileTimes) {
    profile_times.elapsed = profile_times.start.unwrap().elapsed().as_secs_f32();

    println!(
        "[pid {}, cpu {}] real {:?}",
        process::id(),
        sched_getcpu(),
        profile_times.elapsed
    )
}

fn main() {
    let mut profile_times = ProfileTimes::default();

    let mut _x = 1.0f32;
    let mut i = 0_32;
    profile_start(&mut profile_times, &format!("{} fmuls", NUM_MULS));
    while i < NUM_MULS {
        // avoid Rust-specific iterator overhead
        _x *= 1.1f32;
        i += 1;
    }
    profile_log(&mut profile_times);
    black_box(&_x);

    profile_start(
        &mut profile_times,
        &format!("{} allocs of size {}", NUM_ALLOCS, ALLOC_SIZE),
    );
    let layout = Layout::from_size_align(ALLOC_SIZE, ALIGN_SIZE).expect("alignemnt error");
    for _ in 0..NUM_ALLOCS {
        let ptr = unsafe { alloc(layout) }; // intentional leaking like in the exercise template
        black_box(ptr);
    }
    profile_log(&mut profile_times);

    profile_start(&mut profile_times, &format!("sleeping for {:?}", SLEEP_SEC));
    thread::sleep(SLEEP_SEC);
    profile_log(&mut profile_times);
}
