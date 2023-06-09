mod atomics;

use std::thread;

fn main() {

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                let res = atomics::get_data();
                println!("{res}");
            });
        }
    })
}