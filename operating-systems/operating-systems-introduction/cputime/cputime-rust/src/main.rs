use std::alloc::{Layout, alloc};
use std::hint::black_box;
use std::{process, thread, time};
use rustix::thread::{sched_getcpu}; // rustix provides safe bindings to POSIX APIs

const SLEEP_SEC: time::Duration = time::Duration::from_millis(3000);
const NUM_MULS: u32 = 100000000;
const NUM_ALLOCS: u32 = 100000;
const ALLOC_SIZE: usize = 1000;
const ALIGN_SIZE: usize = 16; // default malloc align size on x86-64 Unix systems

// TODO define this struct
#[derive(Default)]
struct ProfileTimes {}

// TODO populate the given struct with starting information
fn profile_start(profile_times: &mut ProfileTimes) {}

// TODO given starting information, compute and log differences to now
fn profile_log(profile_times: &mut ProfileTimes) {
    println!("[pid {}, cpu {}] hey", process::id(), sched_getcpu())
}

fn main() {
    let mut profile_times = ProfileTimes::default();

    // TODO profile doing a bunch of floating point muls
    let mut _x = 1.0f32;
    profile_start(&mut profile_times);
    for _ in 0..NUM_MULS {
        _x *= 1.1;
    }
    black_box(&_x);
    profile_log(&mut profile_times);

    // TODO profile doing a bunch of allocs
    profile_start(&mut profile_times);
    let layout = Layout::from_size_align(ALLOC_SIZE, ALIGN_SIZE).expect("alignemnt error");
    for _ in 0..NUM_ALLOCS {
        let ptr = unsafe { alloc(layout) };     // intentional leaking like in the exercise template
        black_box(ptr);
    }
    profile_log(&mut profile_times);

    // TODO profile sleeping
    profile_start(&mut profile_times);
    thread::sleep(SLEEP_SEC);
    profile_log(&mut profile_times);
}
