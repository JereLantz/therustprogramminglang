fn conv_cel_to_fah(c: f64) -> f64{
    9.0/5.0*c+32.0
}

fn conv_fah_to_cel(f: f64) -> f64{
    5.0/9.0*(f-32.0)
}

fn ask_conversion_type() -> u32 {
    loop {
        let mut choice_input = String::new();
        std::io::stdin().read_line(&mut choice_input)
            .expect("Failed to read the stdin");

        match choice_input.trim().parse() {
            Ok(num) => {
                if num > 0 && num < 3{
                    return num
                }
                println!("Please use the numbers 1 or 2");
            },
            Err(_) => {
                println!("Please use the numbers 1 or 2");
            },
        };
    }
}

fn ask_for_temp_to_conv() -> f64{
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read the stdin when asking for temp to convert");

        match input.trim().parse(){
            Ok(num) => return num,
            Err(_) => println!("Please enter only numbers")
        }
    }
}

fn main() {
    println!("Do you want to convert celsius to fahrenheit, or fahrenheit to celsius?");

    println!("Enter 1 for celsius to fahrenheit conversion");
    println!("Enter 2 for fahrenheit to celsius conversion");

    let conv_type = ask_conversion_type();

    println!("\n");

    if conv_type == 1 {
        println!("Enter the temperature you want to convert (in celsius)");
    }
    else if conv_type == 2 {
        println!("Enter the temperature you want to convert (in fahrenheit)");
    }
    else {
        return;
    }

    let temp_to_conv = ask_for_temp_to_conv();

    println!();
    if conv_type == 1{
        println!("{} celsius is {:.2} in fahrenheit.", temp_to_conv, conv_cel_to_fah(temp_to_conv))
    }
    else if conv_type == 2 {
        println!("{} fahrenheit is {:.2} in celsius.", temp_to_conv, conv_fah_to_cel(temp_to_conv))
    }
}
