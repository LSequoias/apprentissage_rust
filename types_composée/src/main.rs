fn main() {
    les_tableaux();
}


fn les_tableaux() {

    fn exemple_1() {

        let notes: [f32; 5] = [8.5, 12., 16., 20., 9.5];
        
        let mut count = 0;
        
        for i in 0..notes.len() {
            
            if notes[i] <= 10. {
                
                count += 1;
                println!(" {} élève(s) trouvée(s) n'ont pas la moyenne", count);
                
            }
        }
    }

    fn exemple_2() {

        // il est quand même possible de définir un tableau sans valeur si le type est spécifier.
        let vide: [&str; 0] = [""; 0];

        println!("{:#?} du tableau nommée vide", vide);
        // et pas d'erreur, {:?} est pour le debug, rajoutée # pour un effet pretty comme le mongo par exemple..


        // définir un tableau avec d'autre type
        let mut ville: [&str; 3] = ["Paris", "Lyon", "Renne"];
        let boolean: [bool;2] = [true, false];
        println!("ville -> {}\nboolean -> {}\n", ville[1], boolean[0]);


        /*
        * Il n'est pas possible de définir plusieurs types dans un tableau. Les tableaux doit connaitre le nombre de place disponible d'avance
        * Il n'est pas possible de push une valeur dans un tableau, en revanche il nous est possible de modifier une valeur si le tableau est mutable.
        */

        ville[1] = "Carcassonne";

        println!("Changement de Lyon à {}", ville[1]);

    }

    fn exemple_3() {
        // les vecteurs peuvent être eux manipulable (insert, pop, push etc)
        let mut vecteur: Vec<&str> = vec!["HTML", "CSS", "Javascript", "PHP", "SQL", "Rust"];

        // juste pour parcourir notre vecteur
        for vec in &vecteur {
            println!("• {vec}");
        }

        println!(" > {} est un langage structurelle", vecteur[0]);
        
        // Comme d'hab, push à la fin de l'index
        vecteur.push("NoSQL");
        println!(" > {} n'est pas que {}", vecteur[6], vecteur[4]);

        // Insert une donnée au début (index 0)
        vecteur.insert(0, "TypeScript");
        println!(" > {} est une surcouche typée de {}", vecteur[0], vecteur[3]);

        // on veux retirée une donnée..
        vecteur.remove(0);
        println!(" > C'est quand même mieux {}, non ?", vecteur[5]);

        // il y as également la méthode pop() pour retirée le dernier index du vector, au final se n'est pas très éloignée de php
    }



    exemple_1();
    exemple_2();
    exemple_3();
}