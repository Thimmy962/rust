use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    println!("Guess the number!");

    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    
        let guess:u32 = match guess.trim().parse(){
            Ok(number) => number,
            Err(_) =>{println!("Must be an interger");
                    continue;
        },
        };



        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess {guess} is less than the secret_number"),
            Ordering::Greater => println!("Your guess {guess} is greater than the secret_number"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}