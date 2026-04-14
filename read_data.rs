use std::io;

// Este programa lê uma linha de entrada do usuário e a exibe de volta.
// Ele utiliza a função `read_line` para ler a entrada do usuário e a função `trim` para remover quaisquer caracteres de nova linha ou espaços extras.

fn main() {
    let mut entrada = String::new();

    println!("Digite algo:");

    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler a entrada");

    let entrada = entrada.trim(); // remove \n e espaços

    println!("Você digitou: {}", entrada);
}