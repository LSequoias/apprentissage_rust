fn main() {

    //test(256);
    // error compilation
    //other_test("zzzz");
    // error compilation

    // par contre 
    other_test('&');
    // "" et '' sont différent. "" fonctionne pour les string tandis que '' sont pour les char (ASCII, encodage etc)

    let tup : (u8, f32, u8) = (22, 34.8, 100);
    // destructuring
    let (a, b, c) = tup;
    println!("{} : {} : {}", a, b, c);
    println!("première valeur {} et dernière valeur du tuplée {}", tup.0, tup.2);
}

fn test(n:u8) {
    if n < 1 || n > 25 {
        println!("integer overflow ! {}", n);
    } else {
        println!("{}", n);
    }
}

fn other_test(character:char) {
    println!("{}", character);
}



