use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Podaj imię:");
    io::stdin().read_line(&mut input).expect("Błąd wczytywania");
    let first_name = input.trim().to_string();
    input.clear();
    
    println!("Podaj nazwisko:");
    io::stdin().read_line(&mut input).expect("Błąd wczytywania");
    let surname = input.trim().to_string();
    input.clear();
    
    println!("Podaj wiek:");
    io::stdin().read_line(&mut input).expect("Błąd wczytywania");
    let age: u8 = input.trim().parse().expect("Niepoprawny wiek");
    
    println!("Mam na imie {}, moje nazwisko to {}, mam {} lat. Miło mi was poznać.", first_name, surname, age);
}
