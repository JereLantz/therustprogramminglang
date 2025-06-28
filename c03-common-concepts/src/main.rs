fn add69(x: i32) -> i32 {
    // rust palauttaa automaattisesti funktion viimeisen rivin arvon.
    // Jos halutaan returnata aiemmin on mahdollista käyttää return avainsanaa
    // muiden kielien lailla
    x + 69
}

fn expression(x: i32){
    let y = {
        // tämä on expression. Expressionissä ei ole viimeistä puolipistettä ;
        let z = 3;
        // Eli tämä returnaa z + 1
        z + 1
    };

    println!("parameter is {x}");
    println!("y is {y}")
}

fn arrays(){
    // array on aina fixed pituus
    // 5 pitkä array
    let a = [1, 2, 3, 4, 5];

    for n in a {
        println!("{n}");
    }
    // voidaan myös suoraan sanoa mitä tyyppiä halutaan array olevan ja kuinka monta elementtiä
    // sillä on
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    for n in b {
        println!("{n}");
    }
    // Jos arrayn halutaan alustaa monella samalla arvolla se onnistuu seuraavalla syntaxilla
    // tässä alustetaan 5 pituinen array luvulla 3
    let c = [3; 5];
    for n in c {
        println!("{n}")
    }
}

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
    arrays();
    expression(69);

    let number = 420;
    println!("{} plus 69 = {}", number, add69(number))
}
