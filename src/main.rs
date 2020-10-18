use std::io;

fn main() {
    println!("¡Adivina el número!");
    println!("Teclea tu intento.");

    let mut intento = String::new();

    io::stdin().read_line(&mut intento).expect("Failed to read line");

    println!("Tu intento: {}", intento);
}
