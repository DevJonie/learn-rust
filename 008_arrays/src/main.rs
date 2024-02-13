use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August",
    "September", "October", "November", "December"];

    let b: [i32; 5] = [1,2,3,4,5];// [data type; number of elements]

    let c = [3; 5]; // c = [3,3,3,3,3]

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was is not a number");

    let element = months[index];

    println!("The value of the element at index {index} is: {element}");
}
