use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Podaj wartość a:");
    io::stdin().read_line(&mut input).expect("Błąd wczytywania");
    let a: u8 = input.trim().parse().expect("Niepoprawna liczba");
    input.clear();
    
    println!("Podaj wartość b:");
    io::stdin().read_line(&mut input).expect("Błąd wczytywania");
    let b: u8 = input.trim().parse().expect("Niepoprawna liczba");
    
    let c: u16 = a as u16 + b as u16; 
    
    println!("Hello, world!");
    println!("Wynik waszego dodawania to {}", c);
}
