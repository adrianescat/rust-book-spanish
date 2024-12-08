use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// En Rust las variables son inmutables por defecto. Para que sea mutable se le agrega `mut`
// Lo mismo pasa con las referencias, por eso hacemos `&mut guess`
// read_line retorna un ENUM llamado `Result`, en este caso puede ser `Ok` o `Err`. El compilador
// de rust nos va a avisar si no manejamos errores. El `expect` intenta manejar un posible error
// Se usa una depenccia llamada rand, lo agregamos previamente al `cargo.toml`.