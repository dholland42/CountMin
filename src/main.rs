use countmin::CountMin;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    use std::time::Instant;

    let mut mc = CountMin::<2, 1_000>::new();
    println!("Estimated Memory Footprint: {}", CountMin::<2, 1_000>::estimate_size());

    let sent = "this is a test";
    
    let now = Instant::now();
    for _ in 0..4_500_000 {
        for word in sent.split_whitespace() {
            mc.add(&word.to_string())
        }
    }
    let elapsed = now.elapsed();
    for word in sent.split_whitespace() {
        println!("{} -- {}", word, mc.getcount(&word.to_string()).unwrap())
    }
    println!("Junk -- {}", mc.getcount(&"Junk".to_string()).unwrap());
    println!("Program took {:.2?}", elapsed);
}

