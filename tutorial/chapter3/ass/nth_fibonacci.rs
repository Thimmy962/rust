use std::io::stdin;

fn main(){
    let nth_term = get_int("Whats the nth term: ".to_string());
    println!("Your nth term is: {nth_term}");


}

fn get_int(question: String) -> u64{
    let mut int = String::new();

    loop{
        
        println!("{question}");
        
        stdin().read_line(&mut int)
            .expect("something went wrong");

        let int: u64 = match int.trim().parse(){
            Ok(num) =>return num,
            Err(_) =>{
                println!("Not an integer");
                continue;
            }
        };
    }
}


// fn fibonacci(number: u64) -> u64{
//     if number == 0 || number == 1{
//         return number
//     }

//     else{
//         let prev: u64
//     }
// }