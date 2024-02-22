fn main() {
    let s = String::from("heloo");
    takes_ownership(s);
    //This throws an error
    //println!("s after passing to fu, {}", s);

    //////////////////////////////////////////////////////

    let x = 5;
    makes_copy(x);
    //this works smoothly
    println!("x after paasing to fn, {}", x);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}
