// comme un java une fonction main
fn main() {

    /* Par defaut rust rend les variables immuables
    * Un underscore devant le nom de la variable indique à Rust de ne pas lire cette variable lors de la compilation.
    * Sous rust c'est du Snake_case.
    * Il est préférable de typée chaque variable même si cargo et rustc peuvent comprendre le type que l'ont veut employée.
    * Comme php et java, point virgules à chaque instruction.
    */
    let _frontend:&str = "javascript";

    let backend:&str = "php";
    
    // Comme en C et Java, print! est formatée sur une ligne. 
    print!("{} est un langage serveur.", backend);

    // sous bash
    // clear; cargo c; cargo b, cargo r

    fn mutable() {
        // le terme mut permet de rendre une variable mutable, et donc de pouvoir changer sa valeur (pas son type)
        let mut language = "yaml";
        println!("\n{language} peut être utilisée pour Ansible");
        language = "json";
        println!("\n{language} peut être utilisée pour Mongo Atlas comme MariaDB");


        // Le masquage (ou shadowing)
        let language_de_style = "CSS";

        // un block d'instruction
        {
            // le scope est local du coup le masque n'agit pas, return "CSS"
            println!("\n{} peut être utilisée pour le style d'une page.", language_de_style);

        }

        // vue que la variable n'as pas utilisée, ont peu la masquée avec une autre déclaration
        let language_de_style = "SCSS";
        println!("\n{} est un préprocesseur pour le style.", language_de_style);
    }

    fn les_constantes() {
        // comme dans js, java ou php, ils ont le même roles sauf qu'eux ne peuvent pas être mutée
        // de plus nous somme obligée de définir un type d'entrée pour ce genre de variable

        const YEAR: f64 = 28.5;

        println!("\n{YEAR} ans");
    }

    mutable();
}


