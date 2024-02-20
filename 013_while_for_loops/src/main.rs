fn main() {
    //Conditional loops
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");


    //Looping through a Collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("[WHILE Loop] The value is: {}", a[index]);
        index += 1;
    }


    for element in a {
        println!("[FOR Loop] The value is: {element}");
    }


    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFOFF!!!");
}
