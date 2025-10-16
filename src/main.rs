use std::io;

fn main(){
    println!("Weather Converter App");
    println!("Choose the temperature you want to convert to (C or F): ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");
    let temp = temp.trim().to_uppercase();

    if temp == "C" {
        println!("Enter your temperature in Fahrenheit:");
        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit).expect("Failed to read input");
        let fahrenheit: f64 = fahrenheit.trim().parse().expect("Invalid number");
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("Temperature:{}°F", fahrenheit);
        println!("Converted: {}°C", celsius);

    }
    else if temp == "F"{
        println!("Enter your temperature in Celsius:");
        let mut celsius = String::new();
        io::stdin().read_line(&mut celsius).expect("Failed to read input");
        let celsius: f64 = celsius.trim().parse().expect("Invalid number");
        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
        println!("Temperature:{}°C", celsius);
        println!("Converted: {}°F", fahrenheit);
    }
    else {
        println!("Invalid Input.Choose either C or F");
    }
}
