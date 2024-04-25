use std::io::stdin;

fn main(){
    println!("Choose the number in front of the option you want to choose");


    let mut option = String::new();

    loop{

        println!("1. Celsius toFahrenheit\n2. Fahrenheit to Clesius");
        stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: i32 = match option.trim().parse(){
            Ok(number) =>{
                if number == 1 || number == 2{
                    number
                }
                else{
                    println!("Number Should be 1 or 2");
                    continue;
                }
            },
            Err(_) => {
                println!("Must ba a number");
                continue;
            }
        };

            if option == 1{
                cel_to_fahrenheit();

            }

            else if option == 2{
                fahrenheit_to_celsius()
            }
        
            else{
                println!("Choose between 1 and 2");
                continue
            }

            break;
        
        };

}

fn cel_to_fahrenheit(){
    println!("Enter temperature in Celsius:");
            let mut tem = String::new();

            loop {
                tem.clear(); // Clear the previous input
                stdin()
                    .read_line(&mut tem)
                    .expect("Failed to read line");

                let tem: f32 = match tem.trim().parse() {
                    Ok(number) => number,
                    Err(_) => {
                        println!("Must be a number");
                        continue;
                    }
                };

                // Perform Celsius to Fahrenheit conversion
                let fahrenheit = tem * 9.0 / 5.0 + 32.0;
                println!("{} Celsius is {} Fahrenheit", tem, fahrenheit);
                break; // Exit the loop after successful conversion
            }
}


fn fahrenheit_to_celsius(){
    println!("Enter temperature in Fahrenheit:");
    let mut tem = String::new();

    loop {
        tem.clear(); // Clear the previous input
        stdin()
            .read_line(&mut tem)
            .expect("Failed to read line");

        let tem: f32 = match tem.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Must be a number");
                continue;
            }
        };

        // Perform Fahrenheit to Celsius conversion
        let celsius = (tem - 32.0) * 5.0 / 9.0;
        println!("{} Fahrenheit is {} Celsius", tem, celsius);
        break; // Exit the loop after successful conversion
    }
}