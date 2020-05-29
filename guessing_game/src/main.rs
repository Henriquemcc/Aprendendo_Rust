use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main()
{
    println!("Adivinha o número!");

    let numero_secreto=rand::thread_rng().gen_range(1, 101);

    loop
    {
        println!("Por favor, digite a sua suposição.");

        let mut suposicao=String::new();

        io::stdin().read_line(&mut suposicao).expect("Falha ao ler a linha");

        let suposicao: u32 =suposicao.trim().parse().expect("Digite um número!");

        match suposicao.cmp(&numero_secreto)
        {
            Ordering::Less => println!("O número digitad é menor."),
            Ordering::Equal=>
            {
                println!("Você ganhou!");
                break;
            },
            Ordering::Greater=> println!("O número digitado é maior."),
        }
    }
}