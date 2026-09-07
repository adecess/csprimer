use nix::sched::sched_getcpu; // nix provides safe bindings to libc
use nix::sys::resource::{Usage, UsageWho, getrusage};
use nix::sys::time::TimeVal;
use std::hint::black_box;
use std::time::{Duration, Instant};
use std::{process, thread};

const SLEEP_SEC: Duration = Duration::from_millis(3000);
const NUM_MULS: u32 = 100000000;
const NUM_ALLOCS: u32 = 100000;
const ALLOC_SIZE: usize = 1000;

#[derive(Default)]
struct ProfileTimes {
    start: Option<Instant>,
    elapsed: f32,
    usage_start: Option<Usage>,
    usage_end: Option<Usage>,
}

fn profile_start(profile_times: &mut ProfileTimes, log: &str) {
    println!(
        "[pid {}, cpu {}] {}",
        process::id(),
        sched_getcpu().unwrap(),
        log
    );

    profile_times.start = Some(Instant::now());
    profile_times.usage_start = Some(getrusage(UsageWho::RUSAGE_SELF).unwrap());
}

fn profile_log(profile_times: &mut ProfileTimes) {
    profile_times.elapsed = profile_times.start.unwrap().elapsed().as_secs_f32();

    profile_times.usage_end = Some(getrusage(UsageWho::RUSAGE_SELF).unwrap());

    let user_start = duration_from_timeval(profile_times.usage_end.unwrap().user_time());
    let user_end = duration_from_timeval(profile_times.usage_start.unwrap().user_time());
    let user_cpu_time = user_start.saturating_sub(user_end).as_secs_f32();

    let sys_start = duration_from_timeval(profile_times.usage_end.unwrap().system_time());
    let sys_end = duration_from_timeval(profile_times.usage_start.unwrap().system_time());
    let sys_cpu_time = sys_start.saturating_sub(sys_end).as_secs_f32();

    println!(
        "[pid {}, cpu {}] real {:?} user: {:?} sys: {:?}",
        process::id(),
        sched_getcpu().unwrap(),
        profile_times.elapsed,
        user_cpu_time,
        sys_cpu_time
    )
}

fn duration_from_timeval(tv: TimeVal) -> Duration {
    Duration::new(tv.tv_sec() as u64, (tv.tv_usec() * 1000) as u32)
}

fn main() {
    let mut profile_times = ProfileTimes::default();

    let mut x = 1.0f32;
    let mut i = 0_u32;
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
