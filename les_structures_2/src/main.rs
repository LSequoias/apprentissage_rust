use std::collections::HashMap;

struct Monster {
    name: String,
    mui: i32,
    types: String
}


fn create_monster(name:String, mui:i32, types:String) -> Monster {
    Monster {
        // comme Laravel, les props peuvent être rédigé comme ceci si la valeur est identique
        name,
        mui,
        types
    }

}

fn main() {
    let first_instance: Monster = create_monster(String::from("Slime"), 28, String::from("eau"));
    // formatage, tiret sens :  (gauche, centrer, droite) = (<, ^, >); (1, 9) = (min, max)
    println!("{:-^19}", first_instance.name);


    println!("{} {}", first_instance.mui, first_instance.types);

    // pointeur mémoire

    // ref du type, ref de la variable
    let mui_ref:&i32 = &first_instance.mui;

    /* ont teste son poid, <!> size_of_val attend comme argument une ref donc pas de &ma_variable
    * sinon se seras compter comme une ref du pointeur vers un autre pointeur, sa fausse le resultat
    */
    let size_pointer = std::mem::size_of_val(mui_ref);

    println!("Le pointer ->{:p}\nEt son poid -> {} ", mui_ref, size_pointer);
    
    
    // ah c'est génial ça !
    println!("max size : {}", u16::MAX);
    println!("min size : {}", i32::MIN);

    //le_match();
    hash_map();
}


fn le_match() {
    //let test:u16 = 50;

    // bon erreur du à la plage (spread) des nombres en version 1.70, chelou..
    /*match test {
        1..19 => println!("Vous êtes jeune vous savez"),
        18 | 23 => println!("Vous $etes majeurs selon votre pays"),
        24..=32 => println!("Pas si vieux que ça"),
        33..=u16::MAX => println!("L'age n'est plus un problème"),
        _ => println!("Au fond, ont s'en fout pas un peu ?")
    };*/

    // sympas, un switch en plus léger 
}

fn hash_map() {
    let mut person: HashMap<&str, &str> = HashMap::new();
    person.insert("John","Smith");
    person.insert("Jean","Dupont");

    for(k, v ) in person.iter() {
        println!("prénom: {} et nom: {}", k, v);
    }

    let mut pierres_precieuses : HashMap<&str, u8> = HashMap::new();
    // pas d'alias en rust
    let pp = &mut pierres_precieuses;

    pp.insert("diamant", 10);
    pp.insert("coridon", 9);
    pp.insert("rubis", 8);

    for(k, v) in pp.iter() {
        println!("nom: {} et sa densitée: {}", k, v);
    }
}
