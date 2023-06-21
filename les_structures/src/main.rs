fn main() {
    
    // le tuple
    let lambda: (&str, char, i8, f32, bool) = ("salute", 'S', 100, 3. , false );

    println!("première valeur du tuple: {}", lambda.0);

    // desctructurations
    let (a,b,_,_,c) = lambda;

    /* les underscores indique qu 'il y as un bien un argument / assignation ici mais que l'ont ne souhaite pas pour autant s'en servir
    * un peu comme les middlewares d'express
    */

    println!("Première valeur : {}", a);
    println!("Deuxième valeur : {}", b);
    println!("Dernière valeur : {}", c);


    // Les structures, me fait pensez aux interfaces de typescript
    struct Monster {
        name: String,
        index: i32
    }

    let slime: Monster = Monster {
        // bon str et String sont différents aparament
        name: String::from("Slime"),
        index: 01
    };

    println!("créature nommée {}, numéro d'index {}", slime.name, slime.index);
}
