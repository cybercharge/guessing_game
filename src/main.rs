use std::io;
use rand::Rng;

fn main() {
    println!("¡Adivina el número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    println!("El número secreto es: {}", numero_secreto);

    println!("Teclea tu intento.");

    let mut intento = String::new();

    io::stdin().read_line(&mut intento).expect("Failed to read line");

    println!("Tu intento: {}", intento);
}
