use colored::Colorize; // importa a biblioteca colored para colorir a saída
use rand::Rng; // importa o trait Rng para gerar números aleatórios
use std::cmp::Ordering; // importa a enum Ordering para comparações
use std::io; // importa a biblioteca de entrada e saída padrão 

fn main() {
    println!("Adivinhe o número!"); // imprime uma mensagem inicial

    let secret_number = rand::thread_rng().gen_range(1, 10); // gera um número aleatório secreto entre 1 e 10

    println!("O número é {}", secret_number); // imprime o número secreto (apenas para teste)

    loop {
        println!("Por favor, chute um número entre 1 e 10."); // pede ao usuário para chutar um número

        let mut guess = String::new(); // cria uma String mutável para armazenar o chute, por padrão as variáveis são imutáveis

        io::stdin()
            .read_line(&mut guess) // lê uma linha da entrada padrão para a variável guess
            .expect("Falha ao ler a linha"); // trata qualquer erro de leitura

        let guess: u8 = match guess.trim().parse() { // u8 para números de 0 a 255
            Ok(num) => num,
            Err(_) => continue, // ignora o loop atual se houver erro de parse
        };

        println!("Você chutou: {}", guess); // imprime o chute

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Muito pequeno!".red()), // chute muito baixo - cor vermelha
            Ordering::Greater => println!("{}", "Muito alto!".red()), // chute muito alto - cor vermelha
            Ordering::Equal => {
                // chute correto
                println!("{}", "Você acertou!".green());
                break; // sai do loop
            }
        }
    }
}
