// le masquage de variable

fn main() {
    let x = 5;
    // x vaut 5 mais la première variable devient masquée
    let x = x + 5;
    // x vaut désormait 10, sans erreur de compilation
    {
        let x = x + 7;
        // x vaut 17
        println!("val interne : {}", x);
    }

    // x vaut 10
    println!("val externe : {}", x);


    fn nbsp() {
        let space = "ere";
        // len est l'équivalent de lenght de js
        let space = space.len();

        // space 3
        println!("nombre de caractères: {}", space);

        fn erreur_de_type() {
            let phrase = "erreur de type";
            let phrase = 1_000;
            println!("phrase vaut {}", phrase);
            // vue qu'il y as un masque aucune erreur est produite, en revanche
            // rust indique que la variable masqué phrase est undefined
        }

        erreur_de_type();
    }

    nbsp()

}