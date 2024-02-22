fn main(){
    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    return (s, len);
}



///////////////////////////////////////////////////////

fn main_2() {
    let s1 = gives_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("taken and was given back");
    let s2 = takes_and_gives_back(s2);

    println!("s2: {}", s2);
}
fn gives_ownership() -> String {
    let some_string = String::from("giving from a fn");
    return some_string;
}

fn takes_and_gives_back(a_string:String) -> String {
    return a_string;
}



/////////////////////////////////////////////////

fn main_1() {
    let s = String::from("heloo");
    takes_ownership(s);
    //This throws an error
    //println!("s after passing to fu, {}", s);

    ////////////////////////////////////////////////////////////

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
