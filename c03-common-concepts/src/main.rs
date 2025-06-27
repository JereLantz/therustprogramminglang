fn tuples(){
    let tup = (500, 6.4, "asd");

    println!("Tuplen arvot {}, {}, {}", tup.0, tup.1, tup.2);

    let (x,y,z) = tup;

    println!("Destructuroidut arvot {}, {}, {}", x, y, z);

    let mut tup2 = (500, 6.4, "asd");
    tup2.1 = 6.9;

    println!("muokattu tup {}", tup2.1)
}

fn main() {
    tuples();
}
