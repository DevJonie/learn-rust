fn main(){
    println!("Hello, world!");

    another_function(10);

    print_labeled_measurement(2, 'g');

    let five = five();
    println!("Five = {five}");

    let plus_one = plus_one(6);
    println!("Plus one = {plus_one}");
}
//Function params
fn another_function(x:i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32{
    5
}

fn plus_one(x:i32) -> i32{
    return x + 1;
}