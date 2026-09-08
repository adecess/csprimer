use nix::sched::sched_getcpu; // nix provides safe bindings to libc
use nix::sys::resource::{Usage, UsageWho, getrusage};
use nix::sys::time::TimeVal;
use std::alloc::{Layout, alloc};
use std::error::Error;
use std::hint::black_box;
use std::time::{Duration, Instant};
use std::{process, thread};

const SLEEP_SEC: Duration = Duration::from_millis(3000);
const NUM_MULS: u32 = 100000000;
const NUM_ALLOCS: u32 = 100000;
const ALLOC_SIZE: usize = 1000;
const ALIGN_SIZE: usize = 16; // 64-bit Linux minimum alignment

#[derive(Default)]
struct ProfileTimes {
    start: Option<Instant>,
    usage_start: Option<Usage>,
    usage_end: Option<Usage>,
}

fn profile_start(profile_times: &mut ProfileTimes, log: &str) -> Result<(), nix::errno::Errno> {
    println!("[pid {}, cpu {}] {}", process::id(), sched_getcpu()?, log);

    profile_times.start = Some(Instant::now());
    profile_times.usage_start = Some(getrusage(UsageWho::RUSAGE_SELF)?);

    Ok(())
}

fn profile_log(profile_times: &mut ProfileTimes) -> Result<(), nix::errno::Errno> {
    profile_times.usage_end = Some(getrusage(UsageWho::RUSAGE_SELF)?);

    let (Some(start), Some(usage_start), Some(usage_end)) = (
        profile_times.start.as_ref(),
        profile_times.usage_start.as_ref(),
        profile_times.usage_end.as_ref(),
    ) else {
        return Err(nix::errno::Errno::EINVAL);
    };

    let elapsed = start.elapsed().as_secs_f32();

    let user_cpu_time = duration_from_timeval(usage_end.user_time())
        .saturating_sub(duration_from_timeval(usage_start.user_time()))
        .as_secs_f32();

    let sys_cpu_time = duration_from_timeval(usage_end.system_time())
        .saturating_sub(duration_from_timeval(usage_start.system_time()))
        .as_secs_f32();

    println!(
        "[pid {}, cpu {}] real {:?} user: {:?} sys: {:?}",
        process::id(),
        sched_getcpu()?,
        elapsed,
        user_cpu_time,
        sys_cpu_time
    );

    Ok(())
}

fn duration_from_timeval(tv: TimeVal) -> Duration {
    Duration::new(tv.tv_sec() as u64, (tv.tv_usec() * 1000) as u32)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut profile_times = ProfileTimes::default();

    let mut x = 1.0f32;
    let mut i = 0_u32;
    profile_start(&mut profile_times, &format!("{} fmuls", NUM_MULS))?;
    while i < NUM_MULS {
        // avoid Rust-specific iterator overhead
        x *= 1.1f32;
        i += 1;
    }
    black_box(&x);
    profile_log(&mut profile_times)?;

    profile_start(
        &mut profile_times,
        &format!("{} allocs of size {}", NUM_ALLOCS, ALLOC_SIZE),
    )?;

    let layout = Layout::from_size_align(ALLOC_SIZE, ALIGN_SIZE)?;
    for _ in 0..NUM_ALLOCS {
        unsafe {
            let ptr = alloc(layout); // intentional leaking like in the C template
            black_box(ptr);
        }
    }
    profile_log(&mut profile_times)?;

    profile_start(&mut profile_times, &format!("sleeping for {:?}", SLEEP_SEC))?;
    thread::sleep(SLEEP_SEC);
    profile_log(&mut profile_times)?;

    Ok(())
}
