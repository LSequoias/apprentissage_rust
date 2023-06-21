use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {

    les_variables();
    le_projet();
    les_types();


}



fn les_variables() {
    let a = "les variables";
    let _c = "autre chose";
    let b = &a.len();

    println!("\n{:-^66}\n", a);

    println!(
"{b} est le nombre de caractère présent dans la variable a\n
&a est la référence de la variable a\n 
a.len() calcule le nombre de caractères de la chaine appartenant à la variable a\n
un underscore devant la variable indique au compilateur d'ignorée cette même variable\n");

    println!("
cette façon d'appliquée les variables est adaptée au type non composée :\n
valeur de a : {}\nvaleur de b : {}\nvaleur de c : {}", a, b, _c);
}

fn le_projet() {
    let a = "le projet";
    println!("\n{:-^66}\n", a);

    println!("target est l'équivalent du build");
    println!("le fichier Cargo.toml est l'équivalent de package json avec l'option de pouvoir ajoutée des meta");
    println!("cargo peu générée un bundle de teste conçue pour le dev local (test), target reste pour la prod.")
}


fn les_types() {
    let a = "les types";
    println!("\n{:-^66}\n", a);
    let surname = "jean";
    let texte: String = String::from("hello.. nan ne déconnont pas");
    let special_char: char = '~';
    let tableau: [&str; 3] = ["maison", "voiture", "chien"];
    let if_exist: bool = false;
    let mut language: Vec<&str> = vec!["HTML", "CSS", "Javascript", "PHP", "SQL", "Rust"];



    println!("u8 pour unsigned, les nombre non signée\ncela part de {} à {}", u8::MIN, u8::MAX);
    println!("u16 pour unsigned, les nombre non signée\ncela part de {} à {}", u16::MIN, u16::MAX);
    println!("u8 pour unsigned, les nombre non signée\ncela part de {} à {}", u32::MIN, u32::MAX);
    println!("et ceci jusqu'a u128 qui pointe à une valeur max de : {}", u128::MAX);
    println!("\n"); // rholololo le br des familles..
    println!("Même chose pour les nombres signée (négative) i8 : {} et sa plage max: {}",i8::MIN, i8::MAX);
    println!("Si par defaut vous ne mettez pas le type en déclarant la variable\nrust le déduiras si possible");
    println!("\n");
    println!("f32 pour les nombres flottants agit sur les nombres signée comme non signée: {} à {}", f32::MIN, f32::MAX);
    println!("f32 se veux plus précis mais la différent entre f64 est minime car ont n'auras rarement besoin d'aller aussi loin.. : place minimum: {} à la plage maximun {}", f64::MIN, f64::MAX);
    println!("\n");
    println!("la variable surname à comme valeur : {} et est de type : {}", surname, type_of(surname));
    println!("la variable texte à comme valeur : {} et est de type : {}", texte, type_of(&texte));
    println!("String et &str sont différent, l'un est mutable, l'autre non.");
    println!("&str sont des chaines de caractères auxquelles ont connais deja la place allouée");
    println!("tandis que String sort d'une instance et est là pour gérée les chaines dont rust ne connais la taille exacte.");
    println!("la variable special_char à comme valeur : {} et est de type : {}", special_char, type_of(special_char));
    println!("Le type char correspond au chaine pouvant accueillir qu'un caractère");
    println!("et il faut de simple quote pour les char, sinon erreur.");
    println!("\n");
    println!("il y as le type boolean: {}", if_exist);

    println!("\n");
    println!("La variable tableau à comme première valeur : {} et est de type : {}", tableau[0], type_of(tableau));
    println!("Les tableaux ont la même syntaxe que les langages traditionnelles");
    println!("Cependant comme nous somme sur un langage strict, il faut en amont précisée le type des données");
    println!("&str est le type, suivit de la longueur du tableau en question");
    println!("Les tableaux en rust n'accepte qu'un seul et même type à la fois");
    println!("enfin autre grosse différence, oublier push et autre méthode, les tableau en rust sont immuables");
    println!("\n");
    println!("Les vector sont une sorte de tableau améliorée et muttable");
    println!("la première position du vector est similaire aux tableaux, la valeur de l'indexe 0 est {} et sont type est : {}", language[0], type_of(language[0]));
    println!("\n");
    println!("Enfin il y as le type tuple qui lui permet de regroupée plusieurs types");
    println!("bref il y as de quoi s'éclater");

}


