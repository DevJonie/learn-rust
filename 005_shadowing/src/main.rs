fn main (){
    let x = 5;
    println!("The value of x before shadowing is: {x}");
    let x = x + 1;
    println!("The value of x after shadowing: {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    //Spaces example
    let spaces = "     ";
    println!("The value of spaces is: '{spaces}'");
    let spaces = spaces.len();
    println!("The length of spaces is: {spaces}");
}