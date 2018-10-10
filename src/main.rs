extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el número!");

    loop {
        println!("Inserte su numero");
        let mut _numero_insertado = String::new();
        io::stdin().read_line(&mut _numero_insertado)
            .expect("No se ha leido correctamente su numero");
        let _numero_insertado: u32 = match _numero_insertado.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let _secret_number = rand::thread_rng().gen_range(1, 10);
        println!("El numero secreto es: {}", _secret_number);
        println!("Su numero es: {}", _numero_insertado);

        match _numero_insertado.cmp(&_secret_number) {
        Ordering::Less => println!("Muy pequeño"),
        Ordering::Greater => println!("Intenta con uno pequeño"),
        Ordering::Equal => {
            println!("Enhorabuena! Has acertado");
            break;
            }
        }
    
    }
}
