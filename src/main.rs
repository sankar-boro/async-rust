mod atomics;

fn main() {
    let data = atomics::get_data();
    let key = atomics::get_key();
    println!("{key}\n{data}");
}