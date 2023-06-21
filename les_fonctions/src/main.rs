fn main() {

    let y: u32 = {
        // vue que nous sommes dans un block, ceci est considérée comme une instructions, x + 1 ne dois pas avoir de point virgule.
        let x:u32 = 5;
        x + 1
    };

    println!("x + 1 est retournée vue qu'il sagit d'une instruction et non une déclaration: {}", y);
    // 6

    //déclaration
    let mut k:i8 = 1;

    // instruction, valeur de retoure: 5;
    k = {
        k + 4

    };

    println!("valeur de k -> {}", k);

    calc(5, 6, '*');
    calc(5, 6, '/');
    calc(5, 6, '+');
    calc(5, 6, '-');


}


fn calc(x: i32, y:i32, symbole:char) {

    if symbole == '*' {
        println!("{}", x * y);
    } else if symbole == '/' {
        println!("{}", x / y);
    } else if symbole == '-' {
        println!("{}", x - y);
    } else if symbole == '+' {
        println!("{}", x + y);
    } else {
        println!("erreur de symbole.");
    }
}