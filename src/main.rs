use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Devinez le nombre!");

    let nombre_secret = rand::thread_rng().gen_range(1..101);
    println!("Le nombre secret est {nombre_secret}");

    println!("Veuillez entrer un nombre. ");

    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Echec de la lecture de l'entrée utilisateur");

    let supposition: u32 = supposition.trim().parse().expect().
    println!("votre nombre : {supposition}");

    match supposition.cmp(&nombre_secret) {
        Ordering::Less => println!(C est plus !),
        Ordering::Greater => println!(C est moins !),
        Ordering::Equal => println!(T4A GAGNER BG !),
    }  
}
