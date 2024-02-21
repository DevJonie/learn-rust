fn main() {
    let s1 = String::from("Hello");
    let s2 =s1;

    println!("{}, world!", s1); // this thorws an error as the ownership of the string has been moved to s2.
}
