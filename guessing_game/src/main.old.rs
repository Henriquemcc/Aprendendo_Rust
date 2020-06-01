use std::io;

fn main()
{
    println!("Adivinha o número!");

    println!("Digite a sua suposição.");

    let mut suposicao=String::new();

    io::stdin().read_line(&mut suposicao).expect("Falha ao ler a linha");

    println!("Você supos: {}", suposicao);
}
