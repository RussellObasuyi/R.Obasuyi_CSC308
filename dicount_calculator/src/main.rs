 use std::io;
 fn main() {
    println!("RUSSELL'S DISCOUNT CALCULATOR");
    println!("Enter the total bill of items bought:");
    let mut original_bill = String::new();
    io::stdin().read_line(&mut original_bill).expect("Failed to read input");
    let original_bill: f64 = original_bill.trim().parse().expect("Invalid number");

    if original_bill > 5000.0 && original_bill < 10000.0{
        let  discount10 = 0.10 * original_bill;
        let  discount10_price = original_bill - discount10;
        println!("The Original Bill: {}", original_bill);
        println!("The Discount Applied: 10%" );
        println!("The Final Bill: {} ", discount10_price); 
    }

    else if original_bill > 10000.0 {
        let  discount15 = 0.15 * original_bill;
        let  discount15_price = original_bill - discount15;
        println!("The Original Bill: {}", original_bill);
        println!("The Discount Applied: 15%");
        println!("The Final Bill: {}", discount15_price); 
    }
    else{
        println!("No discount will be applied for the price of items");
    }

    
}
