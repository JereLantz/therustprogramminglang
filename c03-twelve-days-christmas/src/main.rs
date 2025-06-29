fn main() {
    let sent = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
    ];

    let ordinal = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "nineth", "tenth", "eleventh", "twelveth"];


    for i in 0..12{
        println!("On the {} day of christmas my true love sent to me", ordinal[i]);

        for j in (0..i+1).rev(){
            if i > 0 && j == 0{
                print!("and ");
            }

            if j == 0 {
                println!("{}", sent[j]);
            }
            else {
                println!("{},", sent[j]);
            }
        }
        println!("\n");
    }
}
