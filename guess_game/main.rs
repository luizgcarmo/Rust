use::stdio;

fn main() {
    println!("Chute um Numero!");
    println!("digite um numero!");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");
    println!("Voce chutou: {guess}");
}