use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
   println!("Choisissez un nombre");
   let mut choix = String::new();
   loop {

    let error_msg = "Veuillez entrée un nombre.";
    let sk = rand::thread_rng().gen_range(1..=100);
    println!("le nombre secret est : {}", sk);

    io::stdin()
      .read_line(&mut choix)
      .expect(&error_msg);
   
    let choix: u32 = choix.trim().parse().expect(&error_msg);

   println!("Votre nombre est {}", choix);


       match choix.cmp(&sk) {
           Ordering::Less => {
                println!("C'est plus");
                break;
           },
           Ordering::Equal => {
                println!("C'est Gagner");
                break;
           },
           Ordering::Greater => {
                println!("C'est moins");
                break;
           }
        }
    }
}
