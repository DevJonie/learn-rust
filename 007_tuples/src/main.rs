fn main() {
    let tup:(i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("tuple: {a}, {b}, {c}");
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("tuple: {x}, {y}, {z}");
}
