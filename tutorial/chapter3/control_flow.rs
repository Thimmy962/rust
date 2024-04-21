use std::io;

fn main() {
    let mut number: String = String::new();

    println!("Enter a number: ");

    io::stdin()
        .read_line(&mut number)
        .expect("Not a number");

    let number: i32 = number.trim().parse().expect("Not a number");


    let list = [1..=100];
    let mut f:bool = false;

    for a in (1..100).rev() {
        if a == number {
            println!("Number choice is in list");
            f = true;
            break;
        }
    }

    if !f{
        println!("Number choice not in list");
    }
}