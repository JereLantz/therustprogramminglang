use std::io;

fn main() {
    println!("How many fibonacci numbers do you want?");

    let amount_num: u32;

    loop{
        let mut requested_input = String::new();

        io::stdin()
            .read_line(&mut requested_input)
            .expect("Failed to read the stdin");

        amount_num = match requested_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only numbers");
                continue;
            },
        };
        break;
    }

    println!();

    let mut prev_prev_num: u32 = 0;
    let mut prev_num: u32 = 1;

    for n in 0..amount_num{
        let curr_num = prev_num + prev_prev_num;

        println!("{}: {}",n+1, curr_num);

        prev_prev_num = prev_num;
        prev_num = curr_num
    }
}
