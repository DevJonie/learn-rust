fn main() {
    let number = 3;

    if number < 3{
        println!("condition was true");
    }else {
        println!("condition was false");
    }

    if number != 0{
        println!("number was something other thatn zero");
    }

    let number = 4;
    if number % 4 == 0{
        println!("number is divisible by 4");
    }

    let number = if number == 4 { 4 } else { 0 };
    println!("The value is {number}");
}
