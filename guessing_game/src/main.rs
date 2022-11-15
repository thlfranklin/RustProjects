use std::{io, cmp::Ordering};
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop{

        println!("Please, input a number (0-100):");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // different approach for err handlings
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please, type a number!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!(">> Result: Too small"),
            Ordering::Greater => println!(">> Result: Too big"),
            Ordering::Equal => {
                println!(">> Result: Bingo!");
                break;
            }
        }
    }

}
