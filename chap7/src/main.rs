
// struct tuple
#[derive(Debug)]
struct KeyPress(String, char);

// struct classique
#[derive(Debug)]
struct MouseClick {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Exemple {
    EventExist(bool), GetKey(KeyPress), GetCursor(MouseClick)
}

fn main() {

    // ont déclare la variable click et ont lui attribut les valeurs selon la structure classique
    let click = MouseClick {
        x: 150,
        y: 300
    };

    println!("location x : {}\nlocation y : {}", click.x, click.y);

    // Ont déclare la variable key et ont lui donne les valeurs selon la structure tuple
    let key = KeyPress(String::from("Ctrl +"), 'S');

    println!("{} {} permet de sauvegarder", key.0, key.1);


    let e_action = Exemple::EventExist(true);

    let e_key = Exemple::GetKey(key);

    let e_click = Exemple::GetCursor(click);

    println!(" {:#?} \n {:#?} \n {:#?} ", e_action, e_key, e_click);

    println!("{}", u8::MAX);
}
