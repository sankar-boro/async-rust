#[derive(Debug)]
struct MutRef<T>(*mut T);

fn main() {
    let mut y: u32 = 0;
    let x = MutRef(&mut y);
    println!("{:?}", x);
}