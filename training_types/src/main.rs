fn main() {
    /*
    * Les types Scalaires
    */
    let n1 = 255;
    let surname = "Pierrot";
    let is_active = false;
    let price = 3.80;
    let cat = '🐈';
    
    println!("Un type Number -> {n1}\nUn type Litteral/String -> {surname}\nUn type Boolean -> {is_active}\nUn type Float -> {price}\nUn type Char -> {cat}");

    /*
    * La différence entre Char et le Litteral
    */

    // Le Litteral peu contenir un ou plusieurs caractères, tandis que le type Char n'en possède qu'un
    let _maison = "Description d'une superbe maison blablabla";

    // Enfin dernière subtilitée le type Char doit absolument comportée de simple apostrophe (simple quote)

    // pas bon
    let _reference = "P";

    // bon 
    let _autre_ref = 'P';

    // Ce chapitre peu paraitre con.. mais en javascript ou en php, cette mécanique est optionnelle.

    les_octets();
    autre_type();
}


fn les_octets() {
    /*
    * les octets 8, 16, 32, 64, 128, 256
    * calculée les octets: 2^8 = 256 
    */

    let vie: u8 = 255;
    let malus: i8 = -10;
    println!("Une nombre signée -> {vie}\nUn nombre non signée -> {malus}");

    let experience: u16 = 60_000;

    let price2: f32 = 9.80;
    println!("Le type u16 -> {experience}\nLe type flotant en 32 bits -> {price2}");

    /*
    * f32 et f64 sont quasi identique, l'un plus rapide (f32), l'autre plus précis (f64)
    * Les flotants enrôles les nombres non signées
    * Par defaut cargo définit le type flotant par f64 et non f32
    */

    
    let _depassement: u8 = 256;
    // si cette variable est executée, rust déclancheras une erreurs, car en base 8, 256 dépasse sa portée, on parle overflow
}

fn autre_type() {
    let arbre: &str = "Tsuga";
    let validation: bool = true;
    println!("Le type Litteral -> {arbre}\n Le type booleen -> {validation}");
}
