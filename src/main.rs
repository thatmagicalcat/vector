mod vector;
use vector::Vector;

fn main() {
    let mut v = Vector::<u32>::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("first value: {}", v[0]);
    println!("last value: {}\n--- Loop ---", v.pop().unwrap());

    v.iter().for_each(|c| println!("{c}"));
}
