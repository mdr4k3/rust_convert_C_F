use std::io;

fn main() {
    let mut temperature = String::new();
    let mut modifier = String::new();

    println!("Enter temperature value: "); 
    
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line!");
    
    let mut temperature:u32 = temperature.trim().parse().expect("Please, use only numbers!");
    
    println!("Enter a modifier C/F only: ");
    
    io::stdin()
        .read_line(&mut modifier)
        .expect("Failed to read line!");

    let modifier:char = modifier.trim().parse().expect("Please, use only F/C");

    if modifier == 'F' {
        temperature = (temperature - 30) / 2;
        println!("Converting from F to C... Temperature is: {temperature}C");
    } else if modifier == 'C' {
        temperature = temperature * 2 + 30;
        println!("Converting from C to F... Temperature is: {temperature}F");
    } else {
        println!("Sorry, you can use only F/C modifiers.");
    }

}
    
