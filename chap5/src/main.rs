


fn main() {
    // tableau et leurs types
    let t : [i32; 3] = [2, 12, 2];
    // lire en débug :? ou #? pour un pretty 
    println!("{:#?}", t);
    println!("{:?}", t[2]);

    // vector définit avec integer32
    let mut vec: Vec<i32> = Vec::from([10, 23, 50]);

    // ajout de 34 comme troisième valeur
    vec.push(34);
    println!("{}", vec[3]);

    // erreur car A est un char et le vector ne peu avoir que des valeur i32
    //vec.push('A');

    println!("{}", chiffre(200));
    boucle();
    autre_boucle();
}

// retourneras comme type finale u32
fn chiffre(x:u32) -> u32 {
    // il aime pas trop que l'ont définit u8 en param avec une sortie u32
    // ont peu cependant faire x.into() pour permuter le type automatiquement
    // ainsi en rentre en override (pas cool)
    return x;
}

// comme js ta toujours les instructions continue et break 
// petite subtilitée: if a {} est conssidérée comme un bool true et un integer
// si tu ne précise pas "a" cela feras une erreur.
fn autre_boucle() {
    let mut a: u32 = 1;
    const STOP: u32 = 10; 
    loop {
        a += 1;
        if a > STOP {
            println!("stop");
            break;
        } else if a == 7 {
            println!("bientôt finit");
            continue;
        }
        println!("tour {:?}", a)
    }
}

// un peu comme un for(const element of mytab) {}
fn boucle() {
    let a = [7, 9, 20, 30];
    for element in a {
        println!("valeur : {:#?}", element);
    }
    for el in (1..11).rev() {
        println!("{:#?}", el);
    }
}




