use rand::prelude::*;

static mut RNG: Option<ThreadRng> = None;

unsafe fn initialize_rng() {
    if RNG.is_none() {
        RNG = Some(thread_rng());
    }
}

#[inline]
unsafe fn rng() -> &'static mut ThreadRng {
    RNG.as_mut().unwrap()
}

fn main() {
    unsafe {
        initialize_rng();
    }
    let rng = unsafe { rng() };
    for _ in 0..10 {
        println!("{}", rng.gen_range(1..100));
    }
}
