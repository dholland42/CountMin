use countmin::{CountMin, HCountMin};

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    use std::time::Instant;

    let mut mc = CountMin::<2, 10>::default();

    let now = Instant::now();
    for idx in 0..200_000_000 {
        mc.add_idx(idx as u64)
    }
    let elapsed = now.elapsed();
    println!("200_000_000 CMS insertions took {:.2?}", elapsed);

    let mut hmc = HCountMin::<2, 10>::default();
    let now = Instant::now();
    for idx in 0..200_000_000 {
        hmc.add_idx(idx as u64, idx as u64 + 1)
    }
    let elapsed = now.elapsed();
    println!("200_000_000 HCMS insertions took {:.2?}", elapsed);

}

