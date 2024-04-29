use std::thread;
use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let start = Instant::now();
    let handles: Vec<_> = (40..45)
        .map(|n| {
            thread::spawn(move || {
                println!("fib({}) = {}", n, fibonacci(n));
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total Time: {:?}", start.elapsed());
}
