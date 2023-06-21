// Structure Classique 
struct Monster {
    name: String,
    uid: u32,
    exist: bool
}

// Structure Tupple
struct Grade(String, char);

fn main() {

    let grade = Grade(String::from("common"), 'C');
    //format!("*^16");
    let slime = Monster {
        name: String::from("Slime"),
        uid:1,
        exist: true
    };

    println!("name: {}\nindexée? {}\nnumero: {}\nrarity\n{}\n{}", slime.name, slime.exist, slime.uid, grade.0, grade.1);
}
