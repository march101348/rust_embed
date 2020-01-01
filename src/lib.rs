use std::thread;

fn process() {
    let handles: Vec<_> = (1..10).map(|_| {
        thread::spawn(move || {
            let mut x = 0;
            for _ in 1..5000000 {
                x += 1;
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
        h.join().map_err(|_| "Could not join a thread.").unwrap());
    }
}
