use rand::Rng;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Wprowadź liczbę:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Błąd wczytywania");
    
        let guess: u32 = guess.trim().parse().expect("Proszę podać liczbę");
    
        if guess < secret_number {
            println!("Za mało!");
        } else if guess > secret_number {
            println!("Za dużo!");
        } else {
            println!("Gratulacje! Zgadłeś.");
            break;
        }
    }
}
